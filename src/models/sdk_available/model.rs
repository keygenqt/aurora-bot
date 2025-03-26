use std::collections::HashMap;

use crate::models::TraitModel;
use crate::tools::macros::tr;
use crate::tools::utils;
use colored::Colorize;
use human_sort::sort;
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum SdkInstallType {
    Online,
    Offline,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum SdkBuildType {
    BT,
    MB2,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SdkAvailableModel {
    pub url: String,
    pub version_major: String,
    pub version_full: String,
    pub build_type: SdkBuildType,
    pub install_type: SdkInstallType,
}

impl TraitModel for SdkAvailableModel {
    fn get_id(&self) -> String {
        format!(
            "{:x}",
            md5::compute(
                format!(
                    "{}:{}:{}",
                    self.version_full,
                    self.name_build_type(),
                    self.name_install_type()
                )
                .as_bytes()
            )
        )
    }

    fn get_key(&self) -> String {
        format!(
            "{} ({}, {})",
            self.version_full,
            self.name_build_type(),
            self.name_install_type()
        )
    }

    fn print(&self) {
        println!(
            "{}",
            tr!(
                "Аврора SDK: {}\nТип сборки: {}\nТип установки: {}\nСсылка: {}",
                self.version_full.bold().white(),
                self.name_build_type().bold().white(),
                self.name_install_type().bold().white(),
                self.url.to_string().bright_blue(),
            )
        );
    }
}

impl SdkAvailableModel {
    pub fn search() -> Vec<SdkAvailableModel> {
        match Self::search_full() {
            Ok(value) => value,
            Err(_) => vec![],
        }
    }

    pub fn search_filter<T: Fn(&SdkAvailableModel) -> bool>(filter: T) -> Vec<SdkAvailableModel> {
        Self::search().iter().filter(|e| filter(e)).cloned().collect()
    }

    pub fn name_build_type(&self) -> String {
        if self.build_type == SdkBuildType::BT {
            "Build Tools".to_string()
        } else {
            "MB2".to_string()
        }
    }

    pub fn name_install_type(&self) -> String {
        if self.install_type == SdkInstallType::Online {
            "Online".to_string()
        } else {
            "Offline".to_string()
        }
    }

    fn search_full() -> Result<Vec<SdkAvailableModel>, Box<dyn std::error::Error>> {
        let url_files = utils::get_repo_url_sdk();
        // Squash urls by full version
        let mut versions: Vec<String> = vec![];
        let mut version_urls: HashMap<String, Vec<String>> = HashMap::new();
        for url in url_files {
            let version_major = url
                .split("installers/")
                .last()
                .unwrap()
                .split("/")
                .nth(0)
                .unwrap()
                .to_string();
            let re = Regex::new(&format!("{}\\.{}", version_major.replace(".", "\\."), r"\d{1, 3}"));
            let version_full = match re.unwrap().captures(&url) {
                Some(value) => value.get(0).unwrap().as_str().to_string(),
                None => continue,
            };
            if version_urls.contains_key(&version_full) {
                version_urls.get_mut(&version_full).unwrap().push(url);
            } else {
                version_urls.insert(version_full.clone(), [url].to_vec());
                versions.push(version_full);
            }
        }
        // Sort version
        let mut versions = versions.iter().map(|e| e.as_str()).collect::<Vec<&str>>();
        sort(&mut versions);
        let reverse = versions.iter().copied().rev().collect::<Vec<&str>>();
        // Map to model
        let mut ids: Vec<String> = vec![];
        let mut models: Vec<SdkAvailableModel> = vec![];
        for version_full in reverse {
            let urls = version_urls.get(version_full).unwrap().clone();
            for url in urls {
                let version_major = url
                    .split("installers/")
                    .last()
                    .unwrap()
                    .split("/")
                    .nth(0)
                    .unwrap()
                    .to_string();
                let re = Regex::new(&format!("{}\\.{}", version_major.replace(".", "\\."), r"\d{1, 3}"));
                let version_full = match re.unwrap().captures(&url) {
                    Some(value) => value.get(0).unwrap().as_str().to_string(),
                    None => continue,
                };
                let build_type = if url.contains("-asbt-") || url.contains("-BT-") {
                    SdkBuildType::BT
                } else {
                    SdkBuildType::MB2
                };
                let install_type = if url.contains("-offline-") {
                    SdkInstallType::Offline
                } else {
                    SdkInstallType::Online
                };
                let model = SdkAvailableModel {
                    url,
                    version_major,
                    version_full,
                    build_type,
                    install_type,
                };
                let id = model.get_id();
                if ids.contains(&id) {
                    continue;
                }
                ids.push(id);
                models.push(model);
            }
        }
        Ok(models)
    }
}
