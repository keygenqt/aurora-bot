use crate::models::configuration::Config;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PsdkModel {
    pub path: String,
    pub version: String,
}

impl PsdkModel {
    #[allow(dead_code)]
    pub async fn search() -> Result<Vec<PsdkModel>, Box<dyn std::error::Error>> {
        match Config::load_psdks() {
            None => Self::search_full().await,
            Some(config) => Ok(config.iter().map(|e| e.to_model()).collect()),
        }
    }

    pub async fn search_full() -> Result<Vec<PsdkModel>, Box<dyn std::error::Error>> {
        // @todo search psdk by system
        Ok(vec![])
    }
}
