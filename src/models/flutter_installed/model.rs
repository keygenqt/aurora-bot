use colored::Colorize;

use crate::models::TraitModel;
use crate::models::configuration::flutter::FlutterConfig;
use crate::service::command::exec;
use crate::tools::macros::print_info;
use crate::tools::utils;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone)]
pub struct FlutterInstalledModel {
    pub id: String,
    pub dir: String,
    pub flutter: String,
    pub dart: String,
    pub flutter_version: String,
    pub tools_version: String,
    pub dart_version: String,
}

impl FlutterInstalledModel {
    pub fn get_id(flutter: &str) -> String {
        format!("{:x}", md5::compute(flutter.as_bytes()))
    }
}

impl TraitModel for FlutterInstalledModel {
    fn get_id(&self) -> String {
        FlutterInstalledModel::get_id(&self.flutter)
    }

    fn get_key(&self) -> String {
        utils::key_from_path(&self.flutter)
    }

    fn print(&self) {
        let message = format!(
            "Flutter SDK: {}\nDart: {}\nDevTools: {}\nДиректория: {}",
            self.flutter_version.bold().white(),
            self.dart_version.bold().white(),
            self.tools_version.bold().white(),
            self.dir.to_string().white(),
        );
        print_info!(message);
    }
}

impl FlutterInstalledModel {
    pub fn search() -> Vec<FlutterInstalledModel> {
        FlutterConfig::load_models()
    }

    pub fn search_filter<T: Fn(&FlutterInstalledModel) -> bool>(filter: T) -> Vec<FlutterInstalledModel> {
        FlutterConfig::load_models()
            .iter()
            .filter(|e| filter(e))
            .cloned()
            .collect()
    }

    pub fn search_full() -> Result<Vec<FlutterInstalledModel>, Box<dyn std::error::Error>> {
        let mut models: Vec<FlutterInstalledModel> = vec![];
        let flutters_path = utils::search_files_by_home("bin/flutter");
        for flutter in flutters_path {
            let dir = flutter.clone().replace("/bin/flutter", "");
            let output = exec::exec_wait_args(&flutter, ["--version"])?;
            let lines = utils::parse_output(output.stdout);
            let flutter_version = match lines.get(0) {
                Some(line) => line
                    .split("•")
                    .map(|e| e.trim())
                    .collect::<Vec<&str>>()
                    .get(0)
                    .unwrap_or_else(|| &"")
                    .split(" ")
                    .collect::<Vec<&str>>()
                    .get(1)
                    .unwrap_or_else(|| &""),
                None => "",
            };
            let dart_version = match lines.get(3) {
                Some(line) => line
                    .split("•")
                    .map(|e| e.trim())
                    .collect::<Vec<&str>>()
                    .get(1)
                    .unwrap_or_else(|| &"")
                    .split(" ")
                    .collect::<Vec<&str>>()
                    .get(1)
                    .unwrap_or_else(|| &""),
                None => "",
            };
            let tools_version = match lines.get(3) {
                Some(line) => line
                    .split("•")
                    .map(|e| e.trim())
                    .collect::<Vec<&str>>()
                    .get(2)
                    .unwrap_or_else(|| &"")
                    .split(" ")
                    .collect::<Vec<&str>>()
                    .get(1)
                    .unwrap_or_else(|| &""),
                None => "",
            };
            if flutter_version.is_empty() || dart_version.is_empty() || tools_version.is_empty() {
                continue;
            }
            models.push(FlutterInstalledModel {
                id: FlutterInstalledModel::get_id(&flutter),
                dir,
                flutter: flutter.clone(),
                dart: flutter.replace("bin/flutter", "bin/dart").to_string(),
                flutter_version: flutter_version.to_string(),
                tools_version: tools_version.to_string(),
                dart_version: dart_version.to_string(),
            })
        }
        Ok(models)
    }
}
