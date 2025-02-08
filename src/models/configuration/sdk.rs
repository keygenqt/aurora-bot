use crate::models::configuration::Config;
use crate::models::sdk::model::SdkModel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct SdkConfig {
    pub path: String,
    pub version: String,
}

impl SdkConfig {
    pub async fn search() -> Config {
        match SdkModel::search_full().await {
            Ok(models) => Config::Sdks(
                models
                    .iter()
                    .map(|e| SdkConfig {
                        path: e.path.clone(),
                        version: e.version.clone(),
                    })
                    .collect(),
            ),
            Err(_) => Config::Emulators(vec![]),
        }
    }

    pub fn to_model(&self) -> SdkModel {
        SdkModel {
            path: self.path.clone(),
            version: self.version.clone(),
        }
    }
}
