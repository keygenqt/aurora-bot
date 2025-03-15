use colored::Colorize;
use serde::Deserialize;
use serde::Serialize;

use super::session::EmulatorSession;
use super::session::EmulatorSessionType;
use crate::models::TraitModel;
use crate::models::configuration::emulator::EmulatorConfig;
use crate::service::command::exec;
use crate::tools::macros::print_info;
use crate::tools::macros::tr;
use crate::tools::programs;
use crate::tools::utils;

#[derive(Serialize, Deserialize, Clone)]
pub struct EmulatorModel {
    pub dir: String,
    pub key: String,
    pub uuid: String,
    pub name: String,
    pub is_running: bool,
}

impl TraitModel for EmulatorModel {
    fn get_id(&self) -> String {
        format!("{:x}", md5::compute(self.uuid.as_bytes()))
    }

    fn get_key(&self) -> String {
        self.name.clone()
    }

    fn print(&self) {
        let message = format!(
            "Эмулятор: {}\nСтатус: {}\nUUID: {}\nДиректория: {}",
            "VirtualBox".bold().white(),
            (if self.is_running {
                "активен"
            } else {
                "не активен"
            })
            .bold()
            .white(),
            self.uuid.bold().white(),
            self.dir.to_string().bold().white()
        );
        print_info!(message);
    }
}

impl EmulatorModel {
    pub fn session_user(&self) -> Result<EmulatorSession, Box<dyn std::error::Error>> {
        Ok(EmulatorSession::new(EmulatorSessionType::User, &self.key)?)
    }

    #[allow(dead_code)]
    pub fn session_root(&self) -> Result<EmulatorSession, Box<dyn std::error::Error>> {
        Ok(EmulatorSession::new(EmulatorSessionType::Root, &self.key)?)
    }

    pub fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let program = programs::get_vboxmanage()?;
        let output = exec::exec_wait_args(&program, ["startvm", self.uuid.as_str()])?;
        if output.status.success() {
            Ok(())
        } else {
            Err(tr!("запустить эмулятор не удалось"))?
        }
    }

    pub fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        let program = programs::get_vboxmanage()?;
        let output = exec::exec_wait_args(&program, ["controlvm", self.uuid.as_str(), "poweroff"])?;
        if output.status.success() {
            Ok(())
        } else {
            Err(tr!("не удалось остановить эмулятор"))?
        }
    }

    pub fn is_recording(&self) -> bool {
        let program = match programs::get_vboxmanage() {
            Ok(value) => value,
            Err(_) => return false,
        };
        let output = match exec::exec_wait_args(&program, ["showvminfo", &self.uuid]) {
            Ok(value) => value,
            Err(_) => return false,
        };
        let lines = utils::parse_output(output.stdout);
        match utils::config_get_bool(&lines, "Recording enabled:", "yes") {
            Ok(value) => value,
            Err(_) => false,
        }
    }

    pub fn search() -> Vec<EmulatorModel> {
        EmulatorConfig::load_models()
    }

    pub fn search_filter<T: Fn(&EmulatorModel) -> bool>(filter: T) -> Vec<EmulatorModel> {
        EmulatorConfig::load_models()
            .iter()
            .filter(|e| filter(e))
            .cloned()
            .collect()
    }

    pub fn search_full() -> Result<Vec<EmulatorModel>, Box<dyn std::error::Error>> {
        let mut emulators = vec![];
        let program = programs::get_vboxmanage()?;
        let output = exec::exec_wait_args(&program, ["list", "vms"])?;
        let lines = utils::parse_output(output.stdout);
        for line in lines {
            if !line.to_lowercase().contains("aurora") {
                continue;
            }
            if line.to_lowercase().contains("engine") {
                continue;
            }
            let uuid = line
                .split_whitespace()
                .last()
                .unwrap()
                .trim()
                .trim_start_matches('{')
                .trim_end_matches('}');
            let name = line.split_whitespace().next().unwrap().trim().trim_matches('"');
            let output = exec::exec_wait_args(&program, ["showvminfo", &uuid])?;
            let lines = utils::parse_output(output.stdout);
            let is_running = match utils::config_get_bool(&lines, "State:", "running") {
                Ok(value) => value,
                Err(_) => continue,
            };
            let dir = match utils::config_get_string(&lines, "Snapshot folder:", " ") {
                Ok(s) => s
                    .split("/")
                    .skip(1)
                    .take_while(|e| !e.contains("emulator"))
                    .map(|e| format!("/{e}"))
                    .collect::<String>(),
                Err(_) => continue,
            };
            emulators.push(EmulatorModel {
                dir: dir.clone(),
                key: format!("{}/vmshare/ssh/private_keys/sdk", dir),
                uuid: uuid.to_string(),
                name: name.to_string(),
                is_running,
            });
        }
        // Result
        Ok(emulators)
    }
}
