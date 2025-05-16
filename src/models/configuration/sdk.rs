use std::error::Error;

use crate::models::configuration::Config;
use crate::models::sdk_installed::model::SdkInstalledModel;
use crate::service::command::exec;
use crate::tools::macros::crash;
use crate::tools::utils;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct SdkConfig {
    pub id: String,
    pub dir: String,
    pub tools: String,
    pub version: String,
    pub qt_creator_version: String,
    pub qt_version: String,
    pub build_date: String,
}

impl SdkConfig {
    pub fn load_models() -> Vec<SdkInstalledModel> {
        let sdk = Config::load().sdk;
        if sdk.is_empty() {
            let update = Self::search();
            if Config::save_sdk(update.clone()) {
                return update.iter().map(|e| e.to_model()).collect();
            }
        }
        sdk.iter().map(|e| e.to_model()).collect()
    }

    pub fn search() -> Vec<SdkConfig> {
        match SdkInstalledModel::search_full() {
            Ok(models) => models
                .iter()
                .map(|e| SdkConfig {
                    id: e.id.clone(),
                    dir: e.dir.clone(),
                    tools: e.tools.clone(),
                    version: e.version.clone(),
                    qt_creator_version: e.qt_creator_version.clone(),
                    qt_version: e.qt_version.clone(),
                    build_date: e.build_date.clone(),
                })
                .collect(),
            Err(error) => crash!(error),
        }
    }

    pub fn to_model(&self) -> SdkInstalledModel {
        fn _get_is_running(sdk_dir: &String) -> Result<bool, Box<dyn Error>> {
            let output = exec::exec_wait_args("ps", ["ax"])?;
            let lines = utils::parse_output(output.stdout);
            Ok(lines.iter().any(|e| e.contains(&format!("{}/bin/qtcreator", sdk_dir))))
        }
        SdkInstalledModel {
            id: self.id.clone(),
            dir: self.dir.clone(),
            tools: self.tools.clone(),
            version: self.version.clone(),
            qt_creator_version: self.qt_creator_version.clone(),
            qt_version: self.qt_version.clone(),
            build_date: self.build_date.clone(),
            is_running: _get_is_running(&self.dir).unwrap_or_else(|_| false),
        }
    }
}
