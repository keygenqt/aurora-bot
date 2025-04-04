use crate::models::configuration::Config;
use crate::models::flutter_installed::model::FlutterInstalledModel;
use serde::Deserialize;
use serde::Serialize;

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
    pub fn load_models() -> Vec<FlutterInstalledModel> {
        let flutter = Config::load().flutter;
        if flutter.is_empty() {
            let update = Self::search();
            if Config::save_flutter(update.clone()) {
                return update.iter().map(|e| e.to_model()).collect();
            }
        }
        flutter.iter().map(|e| e.to_model()).collect()
    }

    pub fn search() -> Vec<FlutterConfig> {
        match FlutterInstalledModel::search_full() {
            Ok(models) => models
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
            Err(_) => vec![],
        }
    }

    pub fn to_model(&self) -> FlutterInstalledModel {
        FlutterInstalledModel {
            id: FlutterInstalledModel::get_id(&self.flutter),
            dir: self.dir.clone(),
            flutter: self.flutter.clone(),
            dart: self.dart.clone(),
            flutter_version: self.flutter_version.clone(),
            tools_version: self.tools_version.clone(),
            dart_version: self.dart_version.clone(),
        }
    }
}
