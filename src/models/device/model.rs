use serde::{Deserialize, Serialize};

use super::session::{EmulatorSession, EmulatorSessionType};
use crate::models::configuration::emulator::EmulatorConfiguration;
use crate::models::configuration::Configuration;
use crate::{
    service::command::exec,
    utils::{methods, programs},
};

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub enum EmulatorType {
    VirtualBox,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EmulatorModel {
    pub emulator_type: EmulatorType,
    pub uuid: String,
    pub folder: String,
    pub is_running: bool,
}

impl EmulatorModel {
    pub async fn session_user(&self) -> Result<EmulatorSession, Box<dyn std::error::Error>> {
        Ok(EmulatorSession::new(EmulatorSessionType::User, &self.folder).await?)
    }

    #[allow(dead_code)]
    pub async fn session_root(&self) -> Result<EmulatorSession, Box<dyn std::error::Error>> {
        Ok(EmulatorSession::new(EmulatorSessionType::Root, &self.folder).await?)
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
        let emulators = Configuration::load().emulator;
        let models = if emulators.is_empty() {
            Self::search_full().await?
        } else {
            emulators.iter().map(|e| e.to_model()).collect()
        };
        Ok(models)
    }

    pub async fn search_full() -> Result<Vec<EmulatorModel>, Box<dyn std::error::Error>> {
        let mut emulators = vec![];
        let program = programs::get_vboxmanage()?;
        let output = exec::exec_wait_args(&program, ["list", "vms"])?;
        let uuids: Vec<String> = String::from_utf8(output.stdout)?
            .split("\n")
            .map(|e| {
                if !e.to_lowercase().contains("aurora") {
                    return None;
                }
                if e.to_lowercase().contains("engine") {
                    return None;
                }
                Some(e.split("{").skip(1).collect::<String>().replace("}", ""))
            })
            .filter(|e| e.is_some())
            .map(|e| e.unwrap().into())
            .collect();

        for uuid in uuids.iter() {
            // Get vm info
            let output = exec::exec_wait_args(&program, ["showvminfo", uuid])?;

            // Load config
            let key_folder = "Snapshot folder:";
            let key_running = "State:";
            let key_recording = "Recording enabled:";
            let params = methods::config_output_filter_keys(
                output,
                [key_folder, key_running, key_recording],
            )?;
            let is_running = methods::config_get_bool(&params, key_running, "running")?;
            let folder = match methods::config_get_string(&params, key_folder, " ") {
                Ok(s) => s
                    .split("/")
                    .skip(1)
                    .take_while(|e| !e.contains("emulator"))
                    .map(|e| format!("/{e}"))
                    .collect::<String>(),
                Err(_) => Err("не удалось найти ключ")?,
            };
            // Create emulator
            let model = EmulatorModel {
                emulator_type: EmulatorType::VirtualBox,
                uuid: uuid.clone(),
                folder,
                is_running,
            };
            model.save();
            emulators.push(model);
        }
        Ok(emulators)
    }

    fn save(&self) {
        let mut list: Vec<EmulatorConfiguration> = vec![];
        let mut config = Configuration::load();
        if config.emulator.iter().any(|e| e.uuid == self.uuid) {
            for item in config.emulator.iter() {
                if item.uuid == self.uuid {
                    list.push(self.to_config());
                } else {
                    list.push(item.clone());
                }
            }
        } else {
            list.push(self.to_config());
        }
        config.update_emulator(list);
        config.save("эмулятора");
    }

    fn to_config(&self) -> EmulatorConfiguration {
        EmulatorConfiguration {
            emulator_type: self.emulator_type.clone(),
            uuid: self.uuid.clone(),
            folder: self.folder.clone(),
        }
    }
}
