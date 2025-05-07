use std::fs;
use std::path::PathBuf;

use colored::Colorize;

use serde::Deserialize;
use serde::Serialize;

use crate::models::TraitModel;
use crate::models::configuration::device::DeviceConfig;
use crate::models::session::model::SessionModel;
use crate::models::session::model::SessionModelType;
use crate::tools::constants;
use crate::tools::macros::print_info;
use crate::tools::macros::tr;
use crate::tools::utils;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeviceModel {
    pub id: String,
    pub host: String,
    pub path: Option<String>,
    pub pass: Option<String>,
    pub port: u16,
    pub name: String,
    pub version: String,
    pub arch: String,
    pub devel_su: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceUserModel {
    pub host: String,
    pub auth: String,
    pub port: u16,
    pub devel_su: String,
}

impl DeviceModel {
    pub fn get_id(value: &str) -> String {
        format!("{:x}", md5::compute(value.as_bytes()))
    }
}

impl TraitModel for DeviceModel {
    fn get_id(&self) -> String {
        DeviceModel::get_id(&self.host)
    }

    fn get_key(&self) -> String {
        self.host.clone().to_lowercase()
    }

    fn print(&self) {
        let message = format!(
            "Устройство: {}:\nHost: {}\nArch: {}\nAuth: {}",
            self.name.bold().white(),
            self.host.bold().white(),
            self.arch.bold().white(),
            if self.pass.is_some() { "password" } else { "key" }.bold().white(),
        );
        print_info!(message);
    }
}

impl DeviceModel {
    #[allow(dead_code)]
    pub fn session_user(&self) -> Result<SessionModel, Box<dyn std::error::Error>> {
        Self::_session_user(&self.path, &self.pass, &self.host, self.port, &self.devel_su)
    }

    fn _session_user(
        path: &Option<String>,
        pass: &Option<String>,
        host: &String,
        port: u16,
        devel_su: &String,
    ) -> Result<SessionModel, Box<dyn std::error::Error>> {
        if let Some(path) = &path {
            return Ok(SessionModel::new_key(
                SessionModelType::User,
                path,
                &host,
                port,
                Some(devel_su.clone()),
            )?);
        }
        if let Some(pass) = &pass {
            return Ok(SessionModel::new_pass(
                SessionModelType::User,
                pass,
                &host,
                port,
                Some(devel_su.clone()),
            )?);
        }
        Err(tr!("необходимо указать пароль или путь к ключу"))?
    }

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
        // Load user config
        let path = utils::get_file_save_path(constants::DEVICES_CONFIGURATION_FILE);
        // Read user config
        let user_data = Self::load_user_config(&path)?;
        let user_models = match serde_json::from_str::<Vec<DeviceUserModel>>(&user_data) {
            Ok(value) => value,
            Err(_) => Err(tr!("конфигурационный файл заполен не верно"))?,
        };
        // Get data devices
        let mut devices = vec![];
        for user_model in user_models {
            let ssh_key_path = PathBuf::from(&user_model.auth);
            let ssh_key_path = match utils::path_to_absolute(&ssh_key_path) {
                Some(value) => Some(value.to_string_lossy().to_string()),
                None => None,
            };
            let ssh_pass = if ssh_key_path.as_ref().is_none() {
                Some(user_model.auth)
            } else {
                None
            };
            let session = match Self::_session_user(
                &ssh_key_path,
                &ssh_pass,
                &user_model.host,
                user_model.port,
                &user_model.devel_su,
            ) {
                Ok(value) => Some(value),
                Err(_) => None,
            };
            if let Some(session) = session {
                devices.push(DeviceModel {
                    id: DeviceModel::get_id(&user_model.host),
                    host: user_model.host,
                    path: ssh_key_path,
                    pass: ssh_pass,
                    port: user_model.port,
                    name: session.os_name,
                    version: session.os_version,
                    arch: session.arch,
                    devel_su: user_model.devel_su,
                });
            }
        }
        Ok(devices)
    }

    fn load_user_config(path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
        let data = match fs::read_to_string(path) {
            Ok(value) => Some(value),
            Err(_) => None,
        };
        if let Some(data) = data {
            return Ok(data);
        }
        // Add default user config if not exist
        let default_devices = vec![DeviceUserModel {
            host: "192.168.2.15".to_string(),
            auth: "00000".to_string(),
            port: 22,
            devel_su: "00000".to_string(),
        }];
        let value_for_save = serde_json::to_string_pretty(&default_devices)?;
        match fs::write(path, &value_for_save) {
            Ok(_) => Ok(value_for_save),
            Err(error) => Err(error)?,
        }
    }
}
