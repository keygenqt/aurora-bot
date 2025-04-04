use colored::Colorize;

use serde::Deserialize;
use serde::Serialize;

use crate::models::TraitModel;
use crate::models::configuration::device::DeviceConfig;
use crate::tools::macros::print_info;

#[derive(Serialize, Deserialize, Clone)]
pub struct DeviceModel {
    pub id: String,
    pub ip: String,
    pub port: u8,
}

impl DeviceModel {
    pub fn get_id(ip: &str) -> String {
        format!("{:x}", md5::compute(ip.as_bytes()))
    }
}

impl TraitModel for DeviceModel {
    fn get_id(&self) -> String {
        DeviceModel::get_id(&self.ip)
    }

    fn get_key(&self) -> String {
        self.ip.clone().to_lowercase()
    }

    fn print(&self) {
        let message = format!("Устройство: {}:{}", self.ip.bold().white(), self.port.to_string());
        print_info!(message);
    }
}

impl DeviceModel {
    pub fn search() -> Vec<DeviceModel> {
        DeviceConfig::load_models()
    }

    pub fn search_filter<T: Fn(&DeviceModel) -> bool>(filter: T) -> Vec<DeviceModel> {
        DeviceConfig::load_models()
            .iter()
            .filter(|e| filter(e))
            .cloned()
            .collect()
    }

    pub fn search_full() -> Result<Vec<DeviceModel>, Box<dyn std::error::Error>> {
        Ok(vec![])
    }
}
