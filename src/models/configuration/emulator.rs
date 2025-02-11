use crate::models::configuration::Config;
use crate::models::emulator::model::EmulatorModel;
use crate::service::command::exec;
use crate::utils::programs;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct EmulatorConfig {
    pub dir: String,
    pub key: String,
    pub uuid: String,
}

impl EmulatorConfig {
    pub async fn load_models() -> Vec<EmulatorModel> {
        let emulator = Config::load().emulator;
        if emulator.is_empty() {
            let update = Self::search().await;
            if Config::save_emulator(update.clone()) {
                return update.iter().map(|e| e.to_model()).collect();
            }
        }
        emulator.iter().map(|e| e.to_model()).collect()
    }

    pub async fn search() -> Vec<EmulatorConfig> {
        match EmulatorModel::search_full().await {
            Ok(models) => models
                .iter()
                .map(|e| EmulatorConfig {
                    dir: e.dir.clone(),
                    key: e.key.clone(),
                    uuid: e.uuid.clone(),
                })
                .collect(),
            Err(_) => vec![],
        }
    }

    pub fn to_model(&self) -> EmulatorModel {
        fn _is_running(uuid: &String) -> Result<bool, Box<dyn Error>> {
            let program = programs::get_vboxmanage()?;
            let output = exec::exec_wait_args(&program, ["list", "runningvms"])?;
            let uuids: String = String::from_utf8(output.stdout)?;
            Ok(uuids.contains(uuid))
        }
        EmulatorModel {
            dir: self.dir.clone(),
            key: self.key.clone(),
            uuid: self.uuid.clone(),
            is_running: _is_running(&self.uuid).unwrap_or_else(|_| false),
        }
    }
}
