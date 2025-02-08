use crate::models::configuration::Config;
use crate::models::psdk::model::PsdkModel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct PsdkConfig {
    pub path: String,
    pub version: String,
}

impl PsdkConfig {
    pub async fn search() -> Config {
        match PsdkModel::search_full().await {
            Ok(models) => Config::Psdks(
                models
                    .iter()
                    .map(|e| PsdkConfig {
                        path: e.path.clone(),
                        version: e.version.clone(),
                    })
                    .collect(),
            ),
            Err(_) => Config::Emulators(vec![]),
        }
    }

    pub fn to_model(&self) -> PsdkModel {
        PsdkModel {
            path: self.path.clone(),
            version: self.version.clone(),
        }
    }
}
