use crate::models::configuration::Config;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct SdkModel {
    pub path: String,
    pub version: String,
}

impl SdkModel {
    #[allow(dead_code)]
    pub async fn search() -> Result<Vec<SdkModel>, Box<dyn std::error::Error>> {
        match Config::load_sdks() {
            None => Self::search_full().await,
            Some(config) => Ok(config.iter().map(|e| e.to_model()).collect()),
        }
    }

    pub async fn search_full() -> Result<Vec<SdkModel>, Box<dyn std::error::Error>> {
        // @todo search psdk by system
        Ok(vec![])
    }
}
