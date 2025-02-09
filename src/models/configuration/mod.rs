use crate::models::configuration::device::DeviceConfig;
use crate::models::configuration::emulator::EmulatorConfig;
use crate::models::configuration::flutter::FlutterConfig;
use crate::models::configuration::psdk::PsdkConfig;
use crate::models::configuration::sdk::SdkConfig;
use crate::utils::constants::CONFIGURATION_FILE;
use crate::utils::methods::get_file_save;
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Mutex;

pub mod device;
pub mod emulator;
pub mod flutter;
pub mod psdk;
pub mod sdk;

#[derive(Debug)]
struct ConfigState {
    change: bool,
}

// State is change configuration
static STATE: Mutex<ConfigState> = Mutex::new(ConfigState { change: false });

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum Config {
    Devices(Vec<DeviceConfig>),
    Emulators(Vec<EmulatorConfig>),
    Flutters(Vec<FlutterConfig>),
    Psdks(Vec<PsdkConfig>),
    Sdks(Vec<SdkConfig>),
}

impl Config {
    pub fn new() -> Vec<Config> {
        vec![
            Self::Devices(vec![]),
            Self::Emulators(vec![]),
            Self::Psdks(vec![]),
            Self::Sdks(vec![]),
        ]
    }

    pub fn load() -> Vec<Config> {
        fn _exec() -> Result<Vec<Config>, Box<dyn std::error::Error>> {
            let path = get_file_save(CONFIGURATION_FILE);
            let data = match fs::read_to_string(path) {
                Ok(value) => value,
                Err(_) => Err("не удалось прочитать конфигурацию")?,
            };
            match serde_json::from_str::<Vec<Config>>(&data) {
                Ok(config) => Ok(config),
                Err(_) => Err("не удалось получить конфигурацию")?,
            }
        }
        _exec().unwrap_or_else(|_| Config::new())
    }

    // Save configuration to file
    pub fn save(self) -> bool {
        STATE.lock().unwrap().change = false;
        // Get updated config
        let data = match self {
            Config::Devices(list) => Self::update_devices(list),
            Config::Emulators(list) => Self::update_emulators(list),
            Config::Flutters(list) => Self::update_flutters(list),
            Config::Psdks(list) => Self::update_psdks(list),
            Config::Sdks(list) => Self::update_sdks(list),
        };
        // Check is not update
        if !STATE.lock().unwrap().change {
            return false;
        }
        // Save
        fn _exec(config: Vec<Config>) -> Result<(), Box<dyn std::error::Error>> {
            let value_for_save = match serde_json::to_string_pretty(&config) {
                Ok(config) => config,
                Err(_) => Err("не удалось получить конфигурацию")?,
            };
            let path = get_file_save(CONFIGURATION_FILE);
            fs::write(path, value_for_save).expect("не удалось записать файл");
            Ok(())
        }
        match _exec(data) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn load_devices() -> Option<Vec<DeviceConfig>> {
        for item in Self::load() {
            match item {
                Config::Devices(list) => {
                    if list.is_empty() {
                        return None;
                    } else {
                        return Some(list);
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn update_devices(data: Vec<DeviceConfig>) -> Vec<Config> {
        let config = Config::load();
        let mut empty = true;
        let mut update: Vec<Config> = vec![];
        for item in config {
            match item {
                Config::Devices(config) => {
                    empty = false;
                    if config != data {
                        STATE.lock().unwrap().change = true;
                        if !data.is_empty() {
                            update.push(Config::Devices(data.clone()))
                        }
                    } else {
                        update.push(Config::Devices(config))
                    }
                }
                _ => update.push(item),
            }
        }
        if empty && !data.is_empty() {
            STATE.lock().unwrap().change = true;
            update.push(Config::Devices(data))
        }
        update
    }

    pub fn load_emulators() -> Option<Vec<EmulatorConfig>> {
        for item in Self::load() {
            match item {
                Config::Emulators(list) => {
                    if list.is_empty() {
                        return None;
                    } else {
                        return Some(list);
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn update_emulators(data: Vec<EmulatorConfig>) -> Vec<Config> {
        let config = Config::load();
        let mut empty = true;
        let mut update: Vec<Config> = vec![];
        for item in config {
            match item {
                Config::Emulators(config) => {
                    empty = false;
                    if config != data {
                        STATE.lock().unwrap().change = true;
                        if !data.is_empty() {
                            update.push(Config::Emulators(data.clone()))
                        }
                    } else {
                        update.push(Config::Emulators(config))
                    }
                }
                _ => update.push(item),
            }
        }
        if empty && !data.is_empty() {
            STATE.lock().unwrap().change = true;
            update.push(Config::Emulators(data))
        }
        update
    }

    pub fn load_flutters() -> Option<Vec<FlutterConfig>> {
        for item in Self::load() {
            match item {
                Config::Flutters(list) => {
                    if list.is_empty() {
                        return None;
                    } else {
                        return Some(list);
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn update_flutters(data: Vec<FlutterConfig>) -> Vec<Config> {
        let config = Config::load();
        let mut empty = true;
        let mut update: Vec<Config> = vec![];
        for item in config {
            match item {
                Config::Flutters(config) => {
                    empty = false;
                    if config != data {
                        STATE.lock().unwrap().change = true;
                        if !data.is_empty() {
                            update.push(Config::Flutters(data.clone()))
                        }
                    } else {
                        update.push(Config::Flutters(config))
                    }
                }
                _ => update.push(item),
            }
        }
        if empty && !data.is_empty() {
            STATE.lock().unwrap().change = true;
            update.push(Config::Flutters(data))
        }
        update
    }

    pub fn load_psdks() -> Option<Vec<PsdkConfig>> {
        for item in Self::load() {
            match item {
                Config::Psdks(list) => {
                    if list.is_empty() {
                        return None;
                    } else {
                        return Some(list);
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn update_psdks(data: Vec<PsdkConfig>) -> Vec<Config> {
        let config = Config::load();
        let mut empty = true;
        let mut update: Vec<Config> = vec![];
        for item in config {
            match item {
                Config::Psdks(config) => {
                    empty = false;
                    if config != data {
                        STATE.lock().unwrap().change = true;
                        if !data.is_empty() {
                            update.push(Config::Psdks(data.clone()))
                        }
                    } else {
                        update.push(Config::Psdks(config))
                    }
                }
                _ => update.push(item),
            }
        }
        if empty && !data.is_empty() {
            STATE.lock().unwrap().change = true;
            update.push(Config::Psdks(data))
        }
        update
    }

    pub fn load_sdks() -> Option<Vec<SdkConfig>> {
        for item in Self::load() {
            match item {
                Config::Sdks(list) => {
                    if list.is_empty() {
                        return None;
                    } else {
                        return Some(list);
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn update_sdks(data: Vec<SdkConfig>) -> Vec<Config> {
        let config = Config::load();
        let mut empty = true;
        let mut update: Vec<Config> = vec![];
        for item in config {
            match item {
                Config::Sdks(config) => {
                    empty = false;
                    if config != data {
                        STATE.lock().unwrap().change = true;
                        if !data.is_empty() {
                            update.push(Config::Sdks(data.clone()))
                        }
                    } else {
                        update.push(Config::Sdks(config))
                    }
                }
                _ => update.push(item),
            }
        }
        if empty && !data.is_empty() {
            STATE.lock().unwrap().change = true;
            update.push(Config::Sdks(data))
        }
        update
    }
}
