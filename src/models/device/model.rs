use colored::Colorize;

use serde::Deserialize;
use serde::Serialize;

use crate::models::configuration::device::DeviceConfig;
use crate::models::TraitModel;
use crate::tools::macros::print_info;

#[derive(Serialize, Deserialize, Clone)]
pub struct DeviceModel {
    pub ip: String,
    pub port: u8,
}

impl TraitModel for DeviceModel {
    fn get_id(&self) -> String {
        format!("{:x}", md5::compute(self.ip.as_bytes()))
    }

    fn print(&self) {
        let message = format!("Устройство: {}:{}", self.ip.bold().white(), self.port.to_string());
        print_info!(message);
    }
}

#[allow(dead_code)]
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
