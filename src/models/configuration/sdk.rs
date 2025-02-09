use crate::models::configuration::Config;
use crate::models::sdk::model::SdkModel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct SdkConfig {
    pub dir: String,
    pub tools: String,
    pub version: String,
}

impl SdkConfig {
    pub async fn search() -> Config {
        match SdkModel::search_full().await {
            Ok(models) => Config::Sdk(
                models
                    .iter()
                    .map(|e| SdkConfig {
                        dir: e.dir.clone(),
                        tools: e.tools.clone(),
                        version: e.version.clone(),
                    })
                    .collect(),
            ),
            Err(_) => Config::Sdk(vec![]),
        }
    }

    pub async fn search_force() -> Vec<SdkModel> {
        let config = Self::search().await;
        config.clone().save();
        match config {
            Config::Sdk(models) => models.iter().map(|e| e.to_model()).collect(),
            _ => vec![],
        }
    }

    pub fn to_model(&self) -> SdkModel {
        SdkModel {
            dir: self.dir.clone(),
            tools: self.tools.clone(),
            version: self.version.clone(),
        }
    }
}
