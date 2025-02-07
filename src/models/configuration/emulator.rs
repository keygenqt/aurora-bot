use crate::models::emulator::model::{EmulatorModel, EmulatorType};
use crate::service::command::exec;
use crate::utils::programs;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct EmulatorConfiguration {
    pub emulator_type: EmulatorType,
    pub uuid: String,
    pub folder: String,
}

impl EmulatorConfiguration {
    pub fn to_model(&self) -> EmulatorModel {
        fn _is_running(uuid: &String) -> Result<bool, Box<dyn Error>> {
            let program = programs::get_vboxmanage()?;
            let output = exec::exec_wait_args(&program, ["list", "runningvms"])?;
            let uuids: String = String::from_utf8(output.stdout)?;
            Ok(uuids.contains(uuid))
        }
        EmulatorModel {
            emulator_type: self.emulator_type.clone(),
            uuid: self.uuid.clone(),
            folder: self.folder.clone(),
            is_running: _is_running(&self.uuid).unwrap_or_else(|_| false),
        }
    }
}
