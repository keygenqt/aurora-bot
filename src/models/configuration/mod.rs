use crate::models::configuration::device::DeviceConfig;
use crate::models::configuration::emulator::EmulatorConfig;
use crate::models::configuration::flutter::FlutterConfig;
use crate::models::configuration::psdk::PsdkConfig;
use crate::models::configuration::sdk::SdkConfig;
use crate::tools::constants;
use crate::tools::macros::tr;
use crate::tools::utils;
use serde::Deserialize;
use serde::Serialize;
use std::fs;

pub mod device;
pub mod emulator;
pub mod flutter;
pub mod psdk;
pub mod sdk;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Config {
    version: String,
    pub device: Vec<DeviceConfig>,
    pub emulator: Vec<EmulatorConfig>,
    pub flutter: Vec<FlutterConfig>,
    pub psdk: Vec<PsdkConfig>,
    pub sdk: Vec<SdkConfig>,
}

impl Config {
    fn new() -> Config {
        Self {
            version: constants::VERSION_CONFIGURATION.to_string(),
            device: vec![],
            emulator: vec![],
            flutter: vec![],
            psdk: vec![],
            sdk: vec![],
        }
    }

    pub fn load() -> Config {
        fn _exec() -> Result<Config, Box<dyn std::error::Error>> {
            let path = utils::get_file_save_path(constants::CONFIGURATION_FILE);
            let data = match fs::read_to_string(path) {
                Ok(value) => value,
                Err(_) => Err(tr!("не удалось прочитать конфигурацию"))?,
            };
            let config = match serde_json::from_str::<Config>(&data) {
                Ok(config) => config,
                Err(_) => Err(tr!("не удалось получить конфигурацию"))?,
            };
            if config.version != Config::new().version {
                Err(tr!("версия конфигурации изменилась"))?
            }
            Ok(config)
        }
        _exec().unwrap_or_else(|_| Config::new())
    }

    fn save(self) -> bool {
        fn _exec(config: Config) -> Result<(), Box<dyn std::error::Error>> {
            let value_for_save = match serde_json::to_string_pretty(&config) {
                Ok(config) => config,
                Err(_) => Err(tr!("не удалось получить конфигурацию"))?,
            };
            let path = utils::get_file_save_path(constants::CONFIGURATION_FILE);
            match fs::write(path, value_for_save) {
                Ok(_) => Ok(()),
                Err(_) => Err(tr!("не удалось записать файл"))?,
            }
        }
        match _exec(self) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn save_device(list: Vec<DeviceConfig>) -> bool {
        let mut config = Self::load();
        if config.device == list {
            return false;
        }
        config.device = list;
        config.save()
    }

    pub fn save_emulator(list: Vec<EmulatorConfig>) -> bool {
        let mut config = Config::load();
        if config.emulator == list {
            return false;
        }
        config.emulator = list;
        config.save()
    }

    pub fn save_flutter(list: Vec<FlutterConfig>) -> bool {
        let mut config = Config::load();
        if config.flutter == list {
            return false;
        }
        config.flutter = list;
        config.save()
    }

    pub fn save_psdk(list: Vec<PsdkConfig>) -> bool {
        let mut config = Config::load();
        if config.psdk == list {
            return false;
        }
        config.psdk = list;
        config.save()
    }

    pub fn save_sdk(list: Vec<SdkConfig>) -> bool {
        let mut config = Config::load();
        if config.sdk == list {
            return false;
        }
        config.sdk = list;
        config.save()
    }
}
