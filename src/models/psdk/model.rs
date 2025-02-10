use colored::Colorize;

use crate::models::configuration::psdk::PsdkConfig;
use crate::models::TraitModel;
use crate::utils::macros::print_info;
use crate::utils::methods;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone)]
pub struct PsdkModel {
    pub dir: String,
    pub chroot: String,
    pub version: String,
    pub version_id: String,
    pub build: u8,
}

impl TraitModel for PsdkModel {
    fn print(&self) {
        let message = format!(
            "Platform SDK: {}\nДиректория: {}",
            self.version_id.bold().white(),
            self.dir.to_string().bold().white(),
        );
        print_info!(message);
    }
}

impl PsdkModel {
    pub async fn search() -> Vec<PsdkModel> {
        PsdkConfig::load_models().await
    }

    pub async fn search_full() -> Result<Vec<PsdkModel>, Box<dyn std::error::Error>> {
        let mut models: Vec<PsdkModel> = vec![];
        let psdks_path = methods::search_files("aurora_psdk/sdk-chroot");
        for chroot in psdks_path {
            let psdk_dir = chroot.replace("/sdk-chroot", "");
            let psdk_release = psdk_dir.clone() + "/etc/aurora-release";
            let data = match fs::read_to_string(psdk_release) {
                Ok(value) => value
                    .split("\n")
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>(),
                Err(_) => continue,
            };
            let version = match methods::config_get_string(&data, "VERSION", "=") {
                Ok(s) => s,
                Err(_) => continue,
            };
            let version_id = match methods::config_get_string(&data, "VERSION_ID", "=") {
                Ok(s) => s,
                Err(_) => continue,
            };
            let build = match methods::config_get_string(&data, "SAILFISH_BUILD", "=") {
                Ok(s) => s.parse::<u8>().unwrap_or_else(|_| 0),
                Err(_) => continue,
            };
            models.push(PsdkModel {
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
