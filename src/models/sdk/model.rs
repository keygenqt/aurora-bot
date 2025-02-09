use colored::Colorize;

use crate::models::configuration::{sdk::SdkConfig, Config};
use crate::models::TraitModel;
use crate::utils::macros::print_info;
use crate::utils::methods;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone)]
pub struct SdkModel {
    pub dir: String,
    pub tools: String,
    pub version: String,
}

impl TraitModel for SdkModel {
    fn print(&self) {
        let message = format!(
            "Аврора SDK: {}\nДиректория: {}",
            self.version.bold().white(),
            self.dir.to_string().bold().white()
        );
        print_info!(message);
    }
}

impl SdkModel {
    pub async fn search() -> Result<Vec<SdkModel>, Box<dyn std::error::Error>> {
        match Config::load_sdks() {
            None => Ok(SdkConfig::search_force().await),
            Some(config) => Ok(config.iter().map(|e| e.to_model()).collect()),
        }
    }

    pub async fn search_full() -> Result<Vec<SdkModel>, Box<dyn std::error::Error>> {
        let mut models: Vec<SdkModel> = vec![];
        let sdks_path = methods::search_files("SDKMaintenanceTool");
        for tools in sdks_path {
            let sdk_dir = tools.replace("/SDKMaintenanceTool", "");
            let sdk_release = sdk_dir.clone() + "/sdk-release";
            let data = match fs::read_to_string(sdk_release) {
                Ok(value) => value
                    .split("\n")
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>(),
                Err(_) => continue,
            };
            let version = match methods::config_get_string(&data, "SDK_RELEASE", "=") {
                Ok(s) => s,
                Err(_) => continue,
            };
            models.push(SdkModel {
                dir: sdk_dir,
                tools: tools.clone(),
                version,
            });
        }
        Ok(models)
    }
}
