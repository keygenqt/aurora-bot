use colored::Colorize;

use crate::models::TraitModel;
use crate::models::configuration::sdk::SdkConfig;
use crate::service::command::exec;
use crate::tools::macros::print_info;
use crate::tools::utils;
use serde::Deserialize;
use serde::Serialize;
use std::error::Error;
use std::fs;

#[derive(Serialize, Deserialize, Clone)]
pub struct SdkInstalledModel {
    pub id: String,
    pub dir: String,
    pub tools: String,
    pub version: String,
    pub qt_creator_version: String,
    pub qt_version: String,
    pub build_date: String,
}

impl SdkInstalledModel {
    pub fn get_id(dir: &str) -> String {
        format!("{:x}", md5::compute(dir.as_bytes()))
    }
}

impl TraitModel for SdkInstalledModel {
    fn get_id(&self) -> String {
        SdkInstalledModel::get_id(&self.dir)
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

            fn _get_qt_creator_version(sdk_dir: &String) -> Result<String, Box<dyn Error>> {
                let output = exec::exec_wait_args(&format!("{sdk_dir}/bin/qtcreator"), ["-version"])?;
                let lines = utils::parse_output(output.stderr);
                Ok(utils::config_get_string_index(&lines, "Qt Creator", " ", 2)?)
            }

            fn _get_qt_version(sdk_dir: &String) -> Result<String, Box<dyn Error>> {
                let output = exec::exec_wait_args(&format!("{sdk_dir}/bin/qtcreator"), ["-version"])?;
                let lines = utils::parse_output(output.stderr);
                Ok(utils::config_get_string_index(&lines, "Qt Creator", " ", -1)?)
            }

            fn _get_build_date(sdk_dir: &String) -> Result<String, Box<dyn Error>> {
                let output = exec::exec_wait_args(&format!("{sdk_dir}/SDKMaintenanceTool"), ["--version"])?;
                let lines = utils::parse_output(output.stdout);
                Ok(utils::config_get_string_index(&lines, "Build date", ":", -1)?)
            }

            models.push(SdkInstalledModel {
                id: SdkInstalledModel::get_id(&sdk_dir),
                dir: sdk_dir.clone(),
                tools: tools.clone(),
                version,
                qt_creator_version: _get_qt_creator_version(&sdk_dir).unwrap_or_else(|_| "undefined".to_string()),
                qt_version: _get_qt_version(&sdk_dir).unwrap_or_else(|_| "undefined".to_string()),
                build_date: _get_build_date(&sdk_dir).unwrap_or_else(|_| "undefined".to_string()),
            });
        }
        Ok(models)
    }
}
