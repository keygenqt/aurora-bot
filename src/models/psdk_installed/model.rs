use colored::Colorize;
use human_sort::sort;

use crate::models::TraitModel;
use crate::models::configuration::psdk::PsdkConfig;
use crate::models::psdk_target::model::PsdkTargetModel;
use crate::tools::macros::print_info;
use crate::tools::macros::print_warning;
use crate::tools::utils;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PsdkInstalledModel {
    pub id: String,
    pub dir: String,
    pub chroot: String,
    pub version: String,
    pub version_id: String,
    pub build: u8,
    pub home_url: String,
    pub targets: Vec<PsdkTargetModel>,
}

impl PsdkInstalledModel {
    pub fn get_id(chroot: &str) -> String {
        format!("{:x}", md5::compute(chroot.as_bytes()))
    }
}

impl TraitModel for PsdkInstalledModel {
    fn get_id(&self) -> String {
        PsdkInstalledModel::get_id(&self.chroot)
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
    pub fn get_latest() -> Option<PsdkInstalledModel> {
        PsdkConfig::load_models().first().cloned()
    }

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
        return Self::_search_full(true);
    }

    pub fn search_full_without_targets() -> Result<Vec<PsdkInstalledModel>, Box<dyn std::error::Error>> {
        return Self::_search_full(false);
    }

    fn _search_full(is_targets: bool) -> Result<Vec<PsdkInstalledModel>, Box<dyn std::error::Error>> {
        let mut models: Vec<PsdkInstalledModel> = vec![];
        let mut models_by_version: HashMap<String, PsdkInstalledModel> = HashMap::new();
        let mut versions: Vec<String> = vec![];
        let psdks_path = utils::search_files_by_home("aurora_psdk/sdk-chroot");
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
            let home_url = match utils::config_get_string(&data, "HOME_URL", "=") {
                Ok(s) => s,
                Err(_) => continue,
            };
            let targets = if is_targets {
                match PsdkTargetModel::search_full(chroot.clone(), psdk_dir.clone()) {
                    Ok(value) => value,
                    Err(error) => {
                        print_warning!(error);
                        vec![]
                    }
                }
            } else {
                vec![]
            };
            let id = PsdkInstalledModel::get_id(&chroot);
            let model = PsdkInstalledModel {
                id: id.clone(),
                dir: psdk_dir,
                chroot: chroot.clone(),
                version_id: version_id.clone(),
                version,
                build,
                home_url,
                targets,
            };
            let key = format!("{} ({})", version_id, id);
            models_by_version.insert(key.clone(), model);
            versions.push(key.clone());
        }
        // Sort version
        let mut versions = versions.iter().map(|e| e.as_str()).collect::<Vec<&str>>();
        sort(&mut versions);
        let reverse = versions.iter().copied().rev().collect::<Vec<&str>>();
        // Make result
        for version in reverse {
            models.push(models_by_version.get(version).unwrap().clone());
        }
        Ok(models)
    }
}
