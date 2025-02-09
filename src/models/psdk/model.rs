use colored::Colorize;

use crate::models::configuration::{psdk::PsdkConfig, Config};
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

impl PsdkModel {
    pub async fn search() -> Result<Vec<PsdkModel>, Box<dyn std::error::Error>> {
        match Config::load_psdks() {
            None => Ok(PsdkConfig::search_force().await),
            Some(config) => Ok(config.iter().map(|e| e.to_model()).collect()),
        }
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

    pub fn print_list(models: Vec<PsdkModel>) {
        if models.is_empty() {
            print_info!("Аврора Platform SDK не найдены")
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
            "Аврора Platform SDK: {}\nДиректория: {}",
            self.version_id.bold().white(),
            self.dir.to_string().bold().white(),
        );
        print_info!(message);
    }
}
