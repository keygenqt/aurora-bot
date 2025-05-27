use crate::models::configuration::Config;
use crate::models::emulator::model::EmulatorModel;
use crate::service::command::exec;
use crate::tools::macros::crash;
use crate::tools::programs;
use crate::tools::utils;
use serde::Deserialize;
use serde::Serialize;
use std::error::Error;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct EmulatorConfig {
    pub id: String,
    pub dir: String,
    pub key: String,
    pub uuid: String,
    pub name: String,
    pub arch: String,
}

impl EmulatorConfig {
    pub fn load_models() -> Vec<EmulatorModel> {
        let emulator = Config::load().emulator;
        if emulator.is_empty() {
            let update = Self::search();
            if Config::save_emulator(update.clone()) {
                return update.iter().map(|e| e.to_model()).collect();
            }
        }
        emulator.iter().map(|e| e.to_model()).collect()
    }

    pub fn search() -> Vec<EmulatorConfig> {
        match EmulatorModel::search_full() {
            Ok(models) => models
                .iter()
                .map(|e| EmulatorConfig {
                    id: e.id.clone(),
                    dir: e.dir.clone(),
                    key: e.key.clone(),
                    uuid: e.uuid.clone(),
                    name: e.name.clone(),
                    arch: e.arch.clone(),
                })
                .collect(),
            Err(error) => crash!(error),
        }
    }

    pub fn to_model(&self) -> EmulatorModel {
        fn _is_running(uuid: &String) -> Result<bool, Box<dyn Error>> {
            let program = programs::get_vboxmanage()?;
            let output = exec::exec_wait_args(&program, ["list", "runningvms"])?;
            let uuids: String = String::from_utf8(output.stdout)?;
            Ok(uuids.contains(uuid))
        }
        fn _is_record(uuid: &String) -> Result<bool, Box<dyn Error>> {
            let program = programs::get_vboxmanage()?;
            let output = exec::exec_wait_args(&program, ["showvminfo", &uuid])?;
            let lines = utils::parse_output(output.stdout);
            utils::config_get_bool(&lines, "Recording enabled:", "yes")
        }
        fn _get_dimensions(uuid: &String) -> Result<String, Box<dyn Error>> {
            let program = programs::get_vboxmanage()?;
            let output = exec::exec_wait_args(&program, ["showvminfo", uuid])?;
            let lines = utils::parse_output(output.stdout);
            Ok(utils::config_get_string(
                &lines,
                "Video dimensions:",
                "Video dimensions:",
            )?)
        }
        EmulatorModel {
            id: self.id.clone(),
            dir: self.dir.clone(),
            key: self.key.clone(),
            uuid: self.uuid.clone(),
            name: self.name.clone(),
            arch: self.arch.clone(),
            is_running: _is_running(&self.uuid).unwrap_or_else(|_| false),
            is_record: _is_record(&self.uuid).unwrap_or_else(|_| false),
            dimensions: _get_dimensions(&self.uuid).unwrap_or_else(|_| "undefined".to_string()),
        }
    }
}
