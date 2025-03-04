use crate::models::configuration::Config;
use crate::models::sdk_installed::model::SdkInstalledModel;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct SdkConfig {
    pub dir: String,
    pub tools: String,
    pub version: String,
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
                    dir: e.dir.clone(),
                    tools: e.tools.clone(),
                    version: e.version.clone(),
                })
                .collect(),
            Err(_) => vec![],
        }
    }

    pub fn to_model(&self) -> SdkInstalledModel {
        SdkInstalledModel {
            dir: self.dir.clone(),
            tools: self.tools.clone(),
            version: self.version.clone(),
        }
    }
}
