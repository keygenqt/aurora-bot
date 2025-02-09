use colored::Colorize;

use crate::{
    models::configuration::{device::DeviceConfig, Config},
    utils::macros::print_info,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct DeviceModel {
    pub ip: String,
    pub port: u8,
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

    pub fn print_list(models: Vec<DeviceModel>) {
        if models.is_empty() {
            print_info!("устройства не найдены")
        }
        for (index, e) in models.iter().enumerate() {
            if index != 0 {
                println!()
            }
            e.print()
        }
    }

    pub fn print(&self) {
        let message = format!(
            "Устройство: {}:{}",
            self.ip.bold().white(),
            self.port.to_string()
        );
        print_info!(message);
    }
}
