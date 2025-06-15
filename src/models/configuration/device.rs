use crate::models::configuration::Config;
use crate::models::device::model::DeviceModel;
use crate::tools::macros::print_warning;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct DeviceConfig {
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

impl DeviceConfig {
    pub fn load_models() -> Vec<DeviceModel> {
        let device = Config::load().device;
        if device.is_empty() {
            let update = Self::search();
            if Config::save_device(update.clone()) {
                return update.iter().map(|e| e.to_model()).collect();
            }
        }
        device.iter().map(|e| e.to_model()).collect()
    }

    pub fn search() -> Vec<DeviceConfig> {
        match DeviceModel::search_full() {
            Ok(models) => models
                .iter()
                .map(|e| DeviceConfig {
                    id: e.id.clone(),
                    host: e.host.clone(),
                    path: e.path.clone(),
                    pass: e.pass.clone(),
                    port: e.port,
                    name: e.name.clone(),
                    version: e.version.clone(),
                    arch: e.arch.clone(),
                    devel_su: e.devel_su.clone(),
                })
                .collect(),
            Err(error) => {
                print_warning!(error);
                return vec![];
            },
        }
    }

    pub fn to_model(&self) -> DeviceModel {
        let is_available =
            match DeviceModel::device_session_user(&self.path, &self.pass, &self.host, self.port, &self.devel_su) {
                Ok(_) => true,
                Err(_) => false,
            };
        DeviceModel {
            id: DeviceModel::get_id(&self.host),
            host: self.host.clone(),
            path: self.path.clone(),
            pass: self.pass.clone(),
            port: self.port,
            name: self.name.clone(),
            version: self.version.clone(),
            arch: self.arch.clone(),
            devel_su: self.devel_su.clone(),
            is_available,
        }
    }
}
