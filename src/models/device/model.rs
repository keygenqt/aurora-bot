use colored::Colorize;

use crate::{
    models::{configuration::{device::DeviceConfig, Config}, TraitModel},
    utils::macros::print_info,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct DeviceModel {
    pub ip: String,
    pub port: u8,
}

impl TraitModel for DeviceModel {
    fn print(&self) {
        let message = format!(
            "Устройство: {}:{}",
            self.ip.bold().white(),
            self.port.to_string()
        );
        print_info!(message);
    }
}

impl DeviceModel {
    pub async fn search() -> Result<Vec<DeviceModel>, Box<dyn std::error::Error>> {
        match Config::load_devices() {
            None => Ok(DeviceConfig::search_force().await),
            Some(config) => Ok(config.iter().map(|e| e.to_model()).collect()),
        }
    }

    pub async fn search_full() -> Result<Vec<DeviceModel>, Box<dyn std::error::Error>> {
        // @todo search devices by localhost
        Ok(vec![])
    }
}
