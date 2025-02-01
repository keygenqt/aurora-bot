use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{
    app::api::{enums::ClientState, outgoing::Outgoing},
    service::{exec::base::exec_wait_args, ssh::client::SshSession},
    utils::programs,
};

#[derive(Deserialize, Serialize)]
pub enum EmulatorType {
    VirtualBox,
}

#[derive(Serialize, Deserialize)]
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

impl EmulatorModel {
    pub fn search() -> Result<Vec<EmulatorModel>, Box<dyn std::error::Error>> {
        EmulatorModel::search_vb()
    }

    pub async fn start(&self) -> Result<Outgoing, Box<dyn std::error::Error>> {
        let program = programs::get_vboxmanage()?;
        let _ = exec_wait_args(&program, ["startvm", self.uuid.as_str()])?;
        self.ping_connect().await?;
        Ok(Outgoing::emulator_start(ClientState::Success))
    }

    async fn ping_connect(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut ssh = SshSession::connect(
            PathBuf::from(&self.key),
            self.user.clone(),
            (self.host.clone(), self.port),
        )
        .await?;
        ssh.close().await?;
        Ok(())
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
                None => Err("Not found name vm")?,
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
                None => Err("Not found name vm")?,
            };

            let is_running = match params.iter().filter(|e| e.contains("State:")).next() {
                Some(option) => option
                    .split(" ")
                    .skip(1)
                    .collect::<String>()
                    .contains("running"),
                None => Err("Not found name vm")?,
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
                None => Err("Not found name vm")?,
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
