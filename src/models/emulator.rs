use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{
    service::{exec::base::exec_wait_args, ssh::client::SshSession},
    utils::programs,
};

#[derive(Deserialize, Serialize, Clone)]
pub enum EmulatorType {
    VirtualBox,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EmulatorModel {
    pub emulator_type: EmulatorType,
    pub uuid: String,
    pub name: String,
    pub key: String,
    pub is_running: bool,
    pub is_recording: bool,
    pub port: u16,
    pub host: String,
    pub user: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EmulatorData {
    pub name: String,
    pub version: String,
}

#[allow(dead_code)]
pub struct EmulatorSession {
    pub model: EmulatorModel,
    pub data: EmulatorData,
    pub session: SshSession,
}

impl EmulatorModel {
    pub fn search() -> Result<Vec<EmulatorModel>, Box<dyn std::error::Error>> {
        EmulatorModel::search_vb()
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let program = programs::get_vboxmanage()?;
        let output = exec_wait_args(&program, ["startvm", self.uuid.as_str()])?;
        let result = String::from_utf8(output.stdout)?;
        // @todo check localization
        if result.contains("successfully") {
            Ok(())
        } else {
            Err("произошла ошибка запуска")?
        }
    }

    pub async fn session(&self) -> Result<EmulatorSession, Box<dyn std::error::Error>> {
        // Connect data
        let key = PathBuf::from(&self.key);
        let user = self.user.clone();
        let host = self.host.clone();
        let port = self.port;
        // Connect
        let session = SshSession::connect(key, user, (host, port)).await?;
        // Get info emulator
        let response = session.call("cat /etc/os-release").await?;
        let output = match response.get(0) {
            Some(value) => value,
            None => Err("не удалось получить ответ с эмулятора")?,
        };
        // Parse name emulator
        let name = match output
            .split("\n")
            .filter(|e| e.contains("PRETTY_NAME="))
            .next()
        {
            Some(option) => option
                .split("=")
                .skip(1)
                .collect::<String>()
                .trim()
                .replace("\"", "")
                .to_string(),
            None => Err("не найдено название ОС Аврора")?,
        };
        // Parse version emulator
        let version = match output
            .split("\n")
            .filter(|e| e.contains("VERSION_ID="))
            .next()
        {
            Some(option) => option
                .split("=")
                .skip(1)
                .collect::<String>()
                .trim()
                .to_string(),
            None => Err("не найдена версия ОС Аврора")?,
        };
        Ok(EmulatorSession {
            model: self.clone(),
            data: EmulatorData { name, version },
            session,
        })
    }

    fn search_vb() -> Result<Vec<EmulatorModel>, Box<dyn std::error::Error>> {
        let mut emulators = vec![];
        let program = programs::get_vboxmanage()?;
        let default_host = String::from("localhost");
        let default_port = 2223;

        let output = exec_wait_args(&program, ["list", "vms"])?;
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
            let output = exec_wait_args(&program, ["showvminfo", uuid])?;
            let params: Vec<String> = String::from_utf8(output.stdout)?
                .split("\n")
                .map(|e| {
                    if e.contains("Name:") {
                        return Some(e);
                    }
                    if e.contains("State:") {
                        return Some(e);
                    }
                    if e.contains("Snapshot folder:") {
                        return Some(e);
                    }
                    if e.contains("Recording enabled:") {
                        return Some(e);
                    }
                    None
                })
                .filter(|e| e.is_some())
                .map(|e| e.unwrap().into())
                .collect();

            let name = match params.iter().filter(|e| e.contains("Name:")).next() {
                Some(option) => option
                    .split(" ")
                    .skip(1)
                    .collect::<String>()
                    .trim()
                    .to_string(),
                None => Err("не найдено название виртуальной машины")?,
            };

            let folder = match params
                .iter()
                .filter(|e| e.contains("Snapshot folder:"))
                .next()
            {
                Some(option) => option
                    .split(" ")
                    .skip(1)
                    .collect::<String>()
                    .trim()
                    .split("/")
                    .skip(1)
                    .take_while(|e| !e.contains("emulator"))
                    .map(|e| format!("/{e}"))
                    .collect::<String>(),
                None => Err("не найден путь к виртуальной машине")?,
            };

            let is_running = match params.iter().filter(|e| e.contains("State:")).next() {
                Some(option) => option
                    .split(" ")
                    .skip(1)
                    .collect::<String>()
                    .contains("running"),
                None => Err("не найден статус виртуальной машины")?,
            };

            let is_recording = match params
                .iter()
                .filter(|e| e.contains("Recording enabled:"))
                .next()
            {
                Some(option) => option
                    .split(" ")
                    .skip(1)
                    .collect::<String>()
                    .contains("enabled:yes"),
                None => Err("не найден статус записи видео на виртуальной машине")?,
            };

            emulators.push(EmulatorModel {
                emulator_type: EmulatorType::VirtualBox,
                uuid: uuid.clone(),
                name: name.clone(),
                key: format!("{}/vmshare/ssh/private_keys/sdk", folder),
                is_running,
                is_recording,
                port: default_port,
                host: default_host.clone(),
                user: String::from("defaultuser"),
            });
        }

        Ok(emulators)
    }
}
