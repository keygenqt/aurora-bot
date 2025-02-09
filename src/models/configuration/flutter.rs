use crate::models::configuration::Config;
use crate::models::flutter::model::FlutterModel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct FlutterConfig {
    pub dir: String,
    pub flutter: String,
    pub dart: String,
    pub flutter_version: String,
    pub tools_version: String,
    pub dart_version: String,
}

impl FlutterConfig {
    pub async fn search() -> Config {
        match FlutterModel::search_full().await {
            Ok(models) => Config::Flutters(
                models
                    .iter()
                    .map(|e| FlutterConfig {
                        dir: e.dir.clone(),
                        flutter: e.flutter.clone(),
                        dart: e.dart.clone(),
                        flutter_version: e.flutter_version.clone(),
                        tools_version: e.tools_version.clone(),
                        dart_version: e.dart_version.clone(),
                    })
                    .collect(),
            ),
            Err(_) => Config::Flutters(vec![]),
        }
    }

    pub async fn search_force() -> Vec<FlutterModel> {
        let config = Self::search().await;
        config.clone().save();
        match config {
            Config::Flutters(models) => models.iter().map(|e| e.to_model()).collect(),
            _ => vec![],
        }
    }

    pub fn to_model(&self) -> FlutterModel {
        FlutterModel {
            dir: self.dir.clone(),
            flutter: self.flutter.clone(),
            dart: self.dart.clone(),
            flutter_version: self.flutter_version.clone(),
            tools_version: self.tools_version.clone(),
            dart_version: self.dart_version.clone(),
        }
    }
}
