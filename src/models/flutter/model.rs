use colored::Colorize;

use crate::models::configuration::{flutter::FlutterConfig, Config};
use crate::models::TraitModel;
use crate::service::command::exec;
use crate::utils::macros::print_info;
use crate::utils::methods;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct FlutterModel {
    pub dir: String,
    pub flutter: String,
    pub dart: String,
    pub flutter_version: String,
    pub tools_version: String,
    pub dart_version: String,
}

impl TraitModel for FlutterModel {
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

impl FlutterModel {
    pub async fn search() -> Result<Vec<FlutterModel>, Box<dyn std::error::Error>> {
        match Config::load_flutters() {
            None => Ok(FlutterConfig::search_force().await),
            Some(config) => Ok(config.iter().map(|e| e.to_model()).collect()),
        }
    }

    pub async fn search_full() -> Result<Vec<FlutterModel>, Box<dyn std::error::Error>> {
        let mut models: Vec<FlutterModel> = vec![];
        let flutters_path = methods::search_files("bin/flutter");
        for flutter in flutters_path {
            let dir = flutter.clone().replace("/bin/flutter", "");
            let output = exec::exec_wait_args(&flutter, ["--version"])?;
            let lines = methods::parse_output(output.stdout);
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
            models.push(FlutterModel {
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
