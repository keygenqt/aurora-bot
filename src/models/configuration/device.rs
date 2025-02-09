use crate::models::configuration::Config;
use crate::models::device::model::DeviceModel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct DeviceConfig {
    pub ip: String,
    pub port: u8,
}

impl DeviceConfig {
    pub async fn search() -> Config {
        match DeviceModel::search_full().await {
            Ok(models) => Config::Device(
                models
                    .iter()
                    .map(|e| DeviceConfig {
                        ip: e.ip.clone(),
                        port: e.port,
                    })
                    .collect(),
            ),
            Err(_) => Config::Device(vec![]),
        }
    }

    pub async fn search_force() -> Vec<DeviceModel> {
        let config = Self::search().await;
        config.clone().save();
        match config {
            Config::Device(models) => models.iter().map(|e| e.to_model()).collect(),
            _ => vec![],
        }
    }

    pub fn to_model(&self) -> DeviceModel {
        DeviceModel {
            ip: self.ip.clone(),
            port: self.port,
        }
    }
}
