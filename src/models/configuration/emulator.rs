use crate::models::configuration::Config;
use crate::models::emulator::model::{EmulatorModel, EmulatorType};
use crate::service::command::exec;
use crate::utils::programs;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct EmulatorConfig {
    pub emulator_type: EmulatorType,
    pub dir: String,
    pub uuid: String,
}

impl EmulatorConfig {
    pub async fn search() -> Config {
        match EmulatorModel::search_full().await {
            Ok(models) => Config::Emulators(
                models
                    .iter()
                    .map(|e| EmulatorConfig {
                        emulator_type: e.emulator_type.clone(),
                        dir: e.dir.clone(),
                        uuid: e.uuid.clone(),
                    })
                    .collect(),
            ),
            Err(_) => Config::Emulators(vec![]),
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
            emulator_type: self.emulator_type.clone(),
            dir: self.dir.clone(),
            uuid: self.uuid.clone(),
            is_running: _is_running(&self.uuid).unwrap_or_else(|_| false),
        }
    }
}
