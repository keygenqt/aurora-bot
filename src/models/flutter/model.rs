use crate::models::configuration::Config;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct FlutterModel {
    pub path: String,
    pub version: String,
}

impl FlutterModel {
    #[allow(dead_code)]
    pub async fn search() -> Result<Vec<FlutterModel>, Box<dyn std::error::Error>> {
        match Config::load_flutters() {
            None => Self::search_full().await,
            Some(config) => Ok(config.iter().map(|e| e.to_model()).collect()),
        }
    }

    pub async fn search_full() -> Result<Vec<FlutterModel>, Box<dyn std::error::Error>> {
        // @todo search flutter by system
        Ok(vec![])
    }
}
