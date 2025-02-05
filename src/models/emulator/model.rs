use serde::{Deserialize, Serialize};

use crate::{
    service::command::exec,
    utils::{methods, programs},
};

use super::session::{EmulatorSession, EmulatorSessionType};

#[derive(Deserialize, Serialize, Clone)]
pub enum EmulatorType {
    VirtualBox,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EmulatorModel {
    pub emulator_type: EmulatorType,
    pub uuid: String,
    pub folder: String,
    pub is_running: bool,
    pub is_recording: bool,
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

    pub fn search() -> Result<Vec<EmulatorModel>, Box<dyn std::error::Error>> {
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
            let is_recording = methods::config_get_bool(&params, key_recording, "enabled:yes")?;
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
            emulators.push(EmulatorModel {
                emulator_type: EmulatorType::VirtualBox,
                uuid: uuid.clone(),
                folder,
                is_running,
                is_recording,
            });
        }
        Ok(emulators)
    }
}
