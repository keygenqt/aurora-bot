use crate::models::configuration::Config;
use crate::models::flutter::model::FlutterModel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct FlutterConfig {
    pub path: String,
    pub version: String,
}

impl FlutterConfig {
    pub async fn search() -> Config {
        match FlutterModel::search_full().await {
            Ok(models) => Config::Flutters(
                models
                    .iter()
                    .map(|e| FlutterConfig {
                        path: e.path.clone(),
                        version: e.version.clone(),
                    })
                    .collect(),
            ),
            Err(_) => Config::Emulators(vec![]),
        }
    }

    pub fn to_model(&self) -> FlutterModel {
        FlutterModel {
            path: self.path.clone(),
            version: self.version.clone(),
        }
    }
}
