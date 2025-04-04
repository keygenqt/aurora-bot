use crate::models::configuration::Config;
use crate::models::device::model::DeviceModel;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct DeviceConfig {
    pub ip: String,
    pub port: u8,
}

impl DeviceConfig {
    pub fn load_models() -> Vec<DeviceModel> {
        let device = Config::load().device;
        if device.is_empty() {
            let update = Self::search();
            if Config::save_device(update.clone()) {
                return update.iter().map(|e| e.to_model()).collect();
            }
        }
        device.iter().map(|e| e.to_model()).collect()
    }

    pub fn search() -> Vec<DeviceConfig> {
        match DeviceModel::search_full() {
            Ok(models) => models
                .iter()
                .map(|e| DeviceConfig {
                    ip: e.ip.clone(),
                    port: e.port,
                })
                .collect(),
            Err(_) => vec![],
        }
    }

    pub fn to_model(&self) -> DeviceModel {
        DeviceModel {
            id: DeviceModel::get_id(&self.ip),
            ip: self.ip.clone(),
            port: self.port,
        }
    }
}
