use crate::models::configuration::device::DeviceConfiguration;
use crate::models::configuration::emulator::EmulatorConfiguration;
use crate::utils::constants::CONFIGURATION_FILE;
use crate::utils::macros::{print_error, print_info, print_success};
use crate::utils::methods::get_file_save;
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Mutex;

pub mod device;
pub mod emulator;

#[derive(Debug)]
struct ConfigurationState {
    change: bool,
}

// State is change configuration
static STATE: Mutex<ConfigurationState> = Mutex::new(ConfigurationState { change: false });

#[derive(Serialize, Deserialize, Clone)]
pub struct Configuration {
    pub device: Vec<DeviceConfiguration>,
    pub emulator: Vec<EmulatorConfiguration>,
}

impl Configuration {
    // Create default configuration
    pub fn new() -> Configuration {
        Configuration {
            device: vec![],
            emulator: vec![],
        }
    }

    // Update device list configuration
    #[allow(dead_code)]
    pub fn update_device(&mut self, value: Vec<DeviceConfiguration>) -> &mut Configuration {
        if self.device != value {
            STATE.lock().unwrap().change = true;
            self.device = value;
        }
        self
    }

    // Update emulator list configuration
    pub fn update_emulator(&mut self, value: Vec<EmulatorConfiguration>) -> &mut Configuration {
        if self.emulator != value {
            STATE.lock().unwrap().change = true;
            self.emulator = value;
        }
        self
    }

    // Load configuration from file
    pub fn load() -> Configuration {
        fn _exec() -> Result<Configuration, Box<dyn std::error::Error>> {
            let path = get_file_save(CONFIGURATION_FILE);
            let data = match fs::read_to_string(path) {
                Ok(value) => value,
                Err(_) => Err("не удалось прочитать конфигурацию")?
            };
            match serde_json::from_str::<Configuration>(&data) {
                Ok(config) => Ok(config),
                Err(_) => Err("не удалось получить конфигурацию")?,
            }
        }
        _exec().unwrap_or_else(|_| Configuration::new())
    }

    // Save configuration to file
    pub fn save(&self, message_type: &str) {
        if !STATE.lock().unwrap().change {
            let message = format!("конфигурация {} актуальна", message_type);
            print_info!(message);
            return;
        }
        STATE.lock().unwrap().change = false;
        fn _exec(config: &Configuration) -> Result<(), Box<dyn std::error::Error>> {
            let value_for_save = match serde_json::to_string_pretty(config) {
                Ok(config) => config,
                Err(_) => Err("не удалось получить конфигурацию")?,
            };
            let path = get_file_save(CONFIGURATION_FILE);
            fs::write(path, value_for_save).expect("не удалось записать файл");
            Ok(())
        }
        match _exec(&self) {
            Ok(_) => {
                let message = format!("конфигурация {} обновлена", message_type);
                print_success!(message)
            },
            Err(error) => print_error!(error),
        }
    }
}
