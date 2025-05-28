use colored::Colorize;

use crate::models::TraitModel;
use crate::models::sdk_installed::model::SdkInstalledModel;
use crate::models::session::model::SessionModel;
use crate::models::session::model::SessionModelType;
use crate::service::command::exec;
use crate::tools::macros::print_info;
use crate::tools::programs;
use crate::tools::utils;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone)]
pub struct SdkInstalledEngineModel {
    pub id: String,
    pub key: String,
    pub uuid: String,
    pub name: String,
    pub is_running: bool,
}

impl SdkInstalledEngineModel {
    pub fn get_id(dir: &str) -> String {
        format!("{:x}", md5::compute(dir.as_bytes()))
    }
}

impl TraitModel for SdkInstalledEngineModel {
    fn get_id(&self) -> String {
        SdkInstalledEngineModel::get_id(&self.uuid)
    }

    fn get_key(&self) -> String {
        utils::key_from_path(&self.uuid)
    }

    fn print(&self) {
        let message = format!(
            "Аврора SDK: {}\nСтатус: {}\nUUID: {}",
            "Engine".bold().white(),
            (if self.is_running {
                "активен"
            } else {
                "не активен"
            })
            .bold()
            .white(),
            self.uuid.bold().white(),
        );
        print_info!(message);
    }
}

impl SdkInstalledEngineModel {
    pub fn session(&self) -> Result<SessionModel, Box<dyn std::error::Error>> {
        Ok(SessionModel::new_key(
            SessionModelType::MerSdk,
            &self.key,
            &"localhost".to_string(),
            2222,
            None,
        )?)
    }

    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let program = programs::get_vboxmanage()?;
        let output = exec::exec_wait_args(&program, ["startvm", self.uuid.as_str(), "--type", "headless"])?;
        if output.status.success() {
            self.is_running = true;
            Ok(())
        } else {
            Err("запустить engine не удалось")?
        }
    }

    #[allow(dead_code)]
    pub fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let program = programs::get_vboxmanage()?;
        let output = exec::exec_wait_args(&program, ["controlvm", self.uuid.as_str(), "poweroff"])?;
        if output.status.success() {
            self.is_running = false;
            Ok(())
        } else {
            Err("не удалось остановить engine")?
        }
    }

    pub fn get_sdk_engine(sdk: &SdkInstalledModel) -> Result<SdkInstalledEngineModel, Box<dyn std::error::Error>> {
        let program = programs::get_vboxmanage()?;
        let output = exec::exec_wait_args(&program, ["list", "vms"])?;
        let lines = utils::parse_output(output.stdout);
        for line in lines {
            if !line.to_lowercase().contains("engine") || !line.to_lowercase().contains(&sdk.version) {
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
            let dir = match utils::config_get_string(&lines, "Snapshot folder:", ":") {
                Ok(s) => s
                    .split("/")
                    .skip(1)
                    .take_while(|e| !e.contains("mersdk"))
                    .map(|e| format!("/{e}"))
                    .collect::<String>(),
                Err(_) => continue,
            };
            return Ok(SdkInstalledEngineModel {
                id: SdkInstalledEngineModel::get_id(&uuid),
                key: format!("{}/vmshare/ssh/private_keys/sdk", dir),
                uuid: uuid.to_string(),
                name: name.to_string(),
                is_running,
            });
        }
        // Result
        Err("SDK Engine не найден")?
    }
}
