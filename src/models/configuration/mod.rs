use crate::models::configuration::device::DeviceConfig;
use crate::models::configuration::emulator::EmulatorConfig;
use crate::models::configuration::flutter::FlutterConfig;
use crate::models::configuration::psdk::PsdkConfig;
use crate::models::configuration::sdk::SdkConfig;
use crate::utils::constants::CONFIGURATION_FILE;
use crate::utils::methods::get_file_save;
use serde::{Deserialize, Serialize};
use serde_variant::to_variant_name;
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
    Device(Vec<DeviceConfig>),
    Emulator(Vec<EmulatorConfig>),
    Flutter(Vec<FlutterConfig>),
    Psdk(Vec<PsdkConfig>),
    Sdk(Vec<SdkConfig>),
}

impl Config {
    pub fn new() -> Vec<Config> {
        vec![
            Self::Device(vec![]),
            Self::Emulator(vec![]),
            Self::Flutter(vec![]),
            Self::Psdk(vec![]),
            Self::Sdk(vec![]),
        ]
    }

    pub fn load() -> Vec<Config> {
        fn _exec() -> Result<Vec<Config>, Box<dyn std::error::Error>> {
            let path = get_file_save(CONFIGURATION_FILE);
            let data = match fs::read_to_string(path) {
                Ok(value) => value,
                Err(_) => Err("не удалось прочитать конфигурацию")?,
            };
            let config = match serde_json::from_str::<Vec<Config>>(&data) {
                Ok(config) => config,
                Err(_) => Err("не удалось получить конфигурацию")?,
            };
            // Remove duplicates
            let mut names: Vec<&str> = vec![];
            let mut clear: Vec<Config> = vec![];
            for item in config {
                let name = to_variant_name(&item).unwrap();
                if names.contains(&name) {
                    STATE.lock().unwrap().change = true;
                    continue;
                }
                names.push(name);
                clear.push(item);
            }
            Ok(clear)
        }
        _exec().unwrap_or_else(|_| {
            STATE.lock().unwrap().change = true;
            Config::new()
        })
    }

    // Save configuration to file
    pub fn save(self) -> bool {
        // Get updated config
        let data = Self::update_models(self);
        // Check is not update
        if !STATE.lock().unwrap().change {
            return false;
        }
        STATE.lock().unwrap().change = false;
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

    fn update_models(data: Config) -> Vec<Config> {
        let config = Config::load();
        let mut empty = true;
        let mut update: Vec<Config> = vec![];
        for item in config {
            match &item {
                Config::Device(config_list) => {
                    match &data {
                        Config::Device(update_list) => {
                            empty = false;
                            if config_list != update_list {
                                STATE.lock().unwrap().change = true;
                                update.push(Config::Device(update_list.clone()))
                            } else {
                                update.push(item.clone())
                            }
                        }
                        _ => update.push(item.clone())
                    }
                },
                Config::Emulator(config_list) => {
                    match &data {
                        Config::Emulator(update_list) => {
                            empty = false;
                            if config_list != update_list {
                                STATE.lock().unwrap().change = true;
                                update.push(Config::Emulator(update_list.clone()))
                            } else {
                                update.push(item.clone())
                            }
                        }
                        _ => update.push(item.clone())
                    }
                },
                Config::Flutter(config_list) => {
                    match &data {
                        Config::Flutter(update_list) => {
                            empty = false;
                            if config_list != update_list {
                                STATE.lock().unwrap().change = true;
                                update.push(Config::Flutter(update_list.clone()))
                            } else {
                                update.push(item.clone())
                            }
                        }
                        _ => update.push(item.clone())
                    }
                },
                Config::Psdk(config_list) => {
                    match &data {
                        Config::Psdk(update_list) => {
                            empty = false;
                            if config_list != update_list {
                                STATE.lock().unwrap().change = true;
                                update.push(Config::Psdk(update_list.clone()))
                            } else {
                                update.push(item.clone())
                            }
                        }
                        _ => update.push(item.clone())
                    }
                },
                Config::Sdk(config_list) => {
                    match &data {
                        Config::Sdk(update_list) => {
                            empty = false;
                            if config_list != update_list {
                                STATE.lock().unwrap().change = true;
                                update.push(Config::Sdk(update_list.clone()))
                            } else {
                                update.push(item.clone())
                            }
                        }
                        _ => update.push(item.clone())
                    }
                },
            }
        }
        if empty {
            STATE.lock().unwrap().change = true;
            update.push(data)
        }
        update
    }

    pub fn load_devices() -> Option<Vec<DeviceConfig>> {
        for item in Self::load() {
            match item {
                Config::Device(list) => {
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

    pub fn load_emulators() -> Option<Vec<EmulatorConfig>> {
        for item in Self::load() {
            match item {
                Config::Emulator(list) => {
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

    pub fn load_flutters() -> Option<Vec<FlutterConfig>> {
        for item in Self::load() {
            match item {
                Config::Flutter(list) => {
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

    pub fn load_psdks() -> Option<Vec<PsdkConfig>> {
        for item in Self::load() {
            match item {
                Config::Psdk(list) => {
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

    pub fn load_sdks() -> Option<Vec<SdkConfig>> {
        for item in Self::load() {
            match item {
                Config::Sdk(list) => {
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
}
