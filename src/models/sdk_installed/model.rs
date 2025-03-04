use colored::Colorize;

use crate::models::TraitModel;
use crate::models::configuration::sdk::SdkConfig;
use crate::tools::macros::print_info;
use crate::tools::utils;
use serde::Deserialize;
use serde::Serialize;
use std::fs;

#[derive(Serialize, Deserialize, Clone)]
pub struct SdkInstalledModel {
    pub dir: String,
    pub tools: String,
    pub version: String,
}

impl TraitModel for SdkInstalledModel {
    fn get_id(&self) -> String {
        format!("{:x}", md5::compute(self.dir.as_bytes()))
    }

    fn get_key(&self) -> String {
        utils::key_from_path(&self.dir)
    }

    fn print(&self) {
        let message = format!(
            "Аврора SDK: {}\nДиректория: {}",
            self.version.bold().white(),
            self.dir.to_string().bold().white()
        );
        print_info!(message);
    }
}

impl SdkInstalledModel {
    pub fn search() -> Vec<SdkInstalledModel> {
        SdkConfig::load_models()
    }

    pub fn search_filter<T: Fn(&SdkInstalledModel) -> bool>(filter: T) -> Vec<SdkInstalledModel> {
        SdkConfig::load_models().iter().filter(|e| filter(e)).cloned().collect()
    }

    pub fn search_full() -> Result<Vec<SdkInstalledModel>, Box<dyn std::error::Error>> {
        let mut models: Vec<SdkInstalledModel> = vec![];
        let sdks_path = utils::search_files("SDKMaintenanceTool");
        for tools in sdks_path {
            let sdk_dir = tools.replace("/SDKMaintenanceTool", "");
            let sdk_release = sdk_dir.clone() + "/sdk-release";
            let data = match fs::read_to_string(sdk_release) {
                Ok(value) => value.split("\n").map(|e| e.to_string()).collect::<Vec<String>>(),
                Err(_) => continue,
            };
            let version = match utils::config_get_string(&data, "SDK_RELEASE", "=") {
                Ok(s) => s,
                Err(_) => continue,
            };
            models.push(SdkInstalledModel {
                dir: sdk_dir,
                tools: tools.clone(),
                version,
            });
        }
        Ok(models)
    }
}
