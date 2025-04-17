use colored::Colorize;
use human_sort::sort;

use crate::models::psdk_target::model::PsdkTargetModel;
use crate::models::TraitModel;
use crate::models::configuration::psdk::PsdkConfig;
use crate::service::command::exec;
use crate::tools::constants;
use crate::tools::macros::print_info;
use crate::tools::single;
use crate::tools::utils;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
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
        let mut models: Vec<PsdkInstalledModel> = vec![];
        let mut models_by_version: HashMap<String, PsdkInstalledModel> = HashMap::new();
        let mut versions: Vec<String> = vec![];

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
            let home_url = match utils::config_get_string(&data, "HOME_URL", "=") {
                Ok(s) => s,
                Err(_) => continue,
            };
            let targets = match PsdkTargetModel::search_full(chroot.clone(), psdk_dir.clone()) {
                Ok(value) => value,
                Err(_) => [].to_vec(),
            };
            let model = PsdkInstalledModel {
                id: PsdkInstalledModel::get_id(&chroot),
                dir: psdk_dir,
                chroot: chroot.clone(),
                version_id: version_id.clone(),
                version,
                build,
                home_url,
                targets,
            };
            models_by_version.insert(version_id.clone(), model);
            versions.push(version_id);
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

    pub fn package_is_sign(&self, path: &PathBuf) -> bool {
        let output = match exec::exec_wait_args(&self.chroot, ["rpmsign-external", "verify", &path.to_string_lossy()]) {
            Ok(value) => value,
            Err(_) => return false,
        };
        let lines = utils::parse_output(output.stdout);
        !lines.is_empty() && lines.last().unwrap().contains("successfully")
    }

    pub fn package_sign(&self, path: &PathBuf) -> bool {
        let path_key = self.get_regular_key();
        if path_key.is_none() {
            return false;
        }
        let path_cert = self.get_regular_cert();
        if path_cert.is_none() {
            return false;
        }
        let _ = match exec::exec_wait_args(
            &self.chroot,
            [
                "rpmsign-external",
                "sign",
                "--force",
                &format!("--key={}", path_key.unwrap().to_string_lossy()),
                &format!("--cert={}", path_cert.unwrap().to_string_lossy()),
                &path.to_string_lossy(),
            ],
        ) {
            Ok(value) => value,
            Err(_) => return false,
        };
        self.package_is_sign(path)
    }

    fn get_regular_key(&self) -> Option<PathBuf> {
        let path = utils::get_file_save_path(constants::SIGN_REG_KEY);
        if !path.exists() {
            match single::get_request().download_file(constants::SIGN_REG_KEY_URL.to_string(), |_| {}) {
                Ok(value) => match fs::rename(value, &path) {
                    Ok(_) => {}
                    Err(_) => return None,
                },
                Err(_) => return None,
            };
        }
        Some(path)
    }

    fn get_regular_cert(&self) -> Option<PathBuf> {
        let path = utils::get_file_save_path(constants::SIGN_REG_CERT);
        if !path.exists() {
            match single::get_request().download_file(constants::SIGN_REG_CERT_URL.to_string(), |_| {}) {
                Ok(value) => match fs::rename(value, &path) {
                    Ok(_) => {}
                    Err(_) => return None,
                },
                Err(_) => return None,
            };
        }
        Some(path)
    }
}
