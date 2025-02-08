use crate::models::configuration::Config;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct DeviceModel {
    pub ip: String,
    pub port: u8,
}

impl DeviceModel {
    #[allow(dead_code)]
    pub async fn search() -> Result<Vec<DeviceModel>, Box<dyn std::error::Error>> {
        match Config::load_devices() {
            None => Self::search_full().await,
            Some(config) => Ok(config.iter().map(|e| e.to_model()).collect()),
        }
    }

    pub async fn search_full() -> Result<Vec<DeviceModel>, Box<dyn std::error::Error>> {
        // @todo search devices by localhost
        Ok(vec![])
    }
}
