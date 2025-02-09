use colored::Colorize;
use serde::{Deserialize, Serialize};

use super::session::{EmulatorSession, EmulatorSessionType};
use crate::models::configuration::emulator::EmulatorConfig;
use crate::models::configuration::Config;
use crate::models::TraitModel;
use crate::utils::macros::print_info;
use crate::{
    service::command::exec,
    utils::{methods, programs},
};

#[derive(Serialize, Deserialize, Clone)]
pub struct EmulatorModel {
    pub dir: String,
    pub uuid: String,
    pub is_running: bool,
}

impl TraitModel for EmulatorModel {
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
    pub async fn session_user(&self) -> Result<EmulatorSession, Box<dyn std::error::Error>> {
        Ok(EmulatorSession::new(EmulatorSessionType::User, &self.dir).await?)
    }

    #[allow(dead_code)]
    pub async fn session_root(&self) -> Result<EmulatorSession, Box<dyn std::error::Error>> {
        Ok(EmulatorSession::new(EmulatorSessionType::Root, &self.dir).await?)
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let program = programs::get_vboxmanage()?;
        let output = exec::exec_wait_args(&program, ["startvm", self.uuid.as_str()])?;
        if output.status.success() {
            Ok(())
        } else {
            Err("запустить эмулятор не удалось")?
        }
    }

    pub async fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        let program = programs::get_vboxmanage()?;
        let output = exec::exec_wait_args(&program, ["controlvm", self.uuid.as_str(), "poweroff"])?;
        if output.status.success() {
            Ok(())
        } else {
            Err("не удалось остановить эмулятор")?
        }
    }

    pub async fn search() -> Result<Vec<EmulatorModel>, Box<dyn std::error::Error>> {
        match Config::load_emulators() {
            None => Ok(EmulatorConfig::search_force().await),
            Some(config) => Ok(config.iter().map(|e| e.to_model()).collect()),
        }
    }

    pub async fn search_full() -> Result<Vec<EmulatorModel>, Box<dyn std::error::Error>> {
        let mut emulators = vec![];
        let program = programs::get_vboxmanage()?;
        let output = exec::exec_wait_args(&program, ["list", "vms"])?;
        let lines = methods::parse_output(output.stdout);
        for line in lines {
            if !line.to_lowercase().contains("aurora") {
                continue;
            }
            if line.to_lowercase().contains("engine") {
                continue;
            }
            let uuid = line.split("{").skip(1).collect::<String>().replace("}", "");
            let output = exec::exec_wait_args(&program, ["showvminfo", &uuid])?;
            let lines = methods::parse_output(output.stdout);
            let is_running = match methods::config_get_bool(&lines, "State:", "running") {
                Ok(value) => value,
                Err(_) => continue,
            };
            let dir = match methods::config_get_string(&lines, "Snapshot folder:", " ") {
                Ok(s) => s
                    .split("/")
                    .skip(1)
                    .take_while(|e| !e.contains("emulator"))
                    .map(|e| format!("/{e}"))
                    .collect::<String>(),
                Err(_) => continue,
            };
            emulators.push(EmulatorModel {
                dir,
                uuid: uuid.clone(),
                is_running,
            });
        }
        // Result
        Ok(emulators)
    }
}
