use colored::Colorize;

use crate::models::TraitModel;
use crate::models::configuration::psdk::PsdkConfig;
use crate::tools::macros::print_info;
use crate::tools::utils;
use serde::Deserialize;
use serde::Serialize;
use std::fs;

#[derive(Serialize, Deserialize, Clone)]
pub struct PsdkInstalledModel {
    pub dir: String,
    pub chroot: String,
    pub version: String,
    pub version_id: String,
    pub build: u8,
}

impl TraitModel for PsdkInstalledModel {
    fn get_id(&self) -> String {
        format!("{:x}", md5::compute(self.chroot.as_bytes()))
    }

    fn get_key(&self) -> String {
        utils::key_from_path(&self.dir)
    }

    fn print(&self) {
        let message = format!(
            "Platform SDK: {}\nДиректория: {}",
            self.version_id.bold().white(),
            self.dir.to_string().bold().white(),
        );
        print_info!(message);
    }
}

impl PsdkInstalledModel {
    pub fn search() -> Vec<PsdkInstalledModel> {
        PsdkConfig::load_models()
    }

    pub fn search_filter<T: Fn(&PsdkInstalledModel) -> bool>(filter: T) -> Vec<PsdkInstalledModel> {
        PsdkConfig::load_models()
            .iter()
            .filter(|e| filter(e))
            .cloned()
            .collect()
    }

    pub fn search_full() -> Result<Vec<PsdkInstalledModel>, Box<dyn std::error::Error>> {
        let mut models: Vec<PsdkInstalledModel> = vec![];
        let psdks_path = utils::search_files("aurora_psdk/sdk-chroot");
        for chroot in psdks_path {
            let psdk_dir = chroot.replace("/sdk-chroot", "");
            let psdk_release = psdk_dir.clone() + "/etc/aurora-release";
            let data = match fs::read_to_string(psdk_release) {
                Ok(value) => value.split("\n").map(|e| e.to_string()).collect::<Vec<String>>(),
                Err(_) => continue,
            };
            let version = match utils::config_get_string(&data, "VERSION", "=") {
                Ok(s) => s,
                Err(_) => continue,
            };
            let version_id = match utils::config_get_string(&data, "VERSION_ID", "=") {
                Ok(s) => s,
                Err(_) => continue,
            };
            let build = match utils::config_get_string(&data, "SAILFISH_BUILD", "=") {
                Ok(s) => s.parse::<u8>().unwrap_or_else(|_| 0),
                Err(_) => continue,
            };
            models.push(PsdkInstalledModel {
                dir: psdk_dir,
                chroot: chroot.clone(),
                version_id,
                version,
                build,
            });
        }
        Ok(models)
    }
}
