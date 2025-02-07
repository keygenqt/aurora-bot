use serde::{Deserialize, Serialize};

use crate::models::configuration::device::DeviceConfiguration;
use crate::models::configuration::Configuration;

#[derive(Serialize, Deserialize, Clone)]
pub struct DeviceModel {
    pub ip: String,
    pub host: String,
}

impl DeviceModel {
    // @todo
    #[allow(dead_code)]
    pub async fn search() -> Result<Vec<DeviceModel>, Box<dyn std::error::Error>> {
        let devices = Configuration::load().device;
        let models = if devices.is_empty() {
            Self::search_full().await?
        } else {
            devices.iter().map(|e| e.to_model()).collect()
        };
        Ok(models)
    }

    // @todo
    #[allow(dead_code)]
    pub async fn search_full() -> Result<Vec<DeviceModel>, Box<dyn std::error::Error>> {
        Ok(vec![])
    }

    // @todo
    #[allow(dead_code)]
    fn save(&self) {
        let mut list: Vec<DeviceConfiguration> = vec![];
        let mut config = Configuration::load();
        if config.device.iter().any(|e| e.ip == self.ip) {
            for item in config.device.iter() {
                if item.ip == self.ip {
                    list.push(self.to_config());
                } else {
                    list.push(item.clone());
                }
            }
        } else {
            list.push(self.to_config());
        }
        config.update_device(list);
        config.save("эмулятора");
    }

    fn to_config(&self) -> DeviceConfiguration {
        DeviceConfiguration {
            ip: self.ip.clone(),
            host: self.host.clone(),
        }
    }
}
