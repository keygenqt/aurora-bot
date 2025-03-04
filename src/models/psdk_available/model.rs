use std::collections::HashMap;

use crate::models::TraitModel;
use crate::tools::macros::tr;
use crate::tools::utils;
use colored::Colorize;
use human_sort::sort;
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone)]
pub struct PsdkAvailableModel {
    pub version_major: String,
    pub version_full: String,
    pub urls: Vec<String>,
}

impl TraitModel for PsdkAvailableModel {
    fn get_id(&self) -> String {
        format!("{:x}", md5::compute(self.version_full.as_bytes()))
    }

    fn get_key(&self) -> String {
        self.version_full.clone()
    }

    fn print(&self) {
        let mut message_lines: Vec<String> = vec![tr!("Platform SDK: {}", self.version_full.bold().white())];
        for url in &self.urls {
            if url.to_lowercase().contains("chroot") {
                message_lines.push(tr!("Chroot: {}", url.bright_blue()));
            }
            if url.to_lowercase().contains("tooling") {
                message_lines.push(tr!("Tooling: {}", url.bright_blue()));
            }
        }
        for url in &self.urls {
            if url.to_lowercase().contains("target") && url.contains("aarch64") {
                message_lines.push(tr!("Target (aarch64): {}", url.bright_blue()));
            }
            if url.to_lowercase().contains("target") && url.contains("armv7hl") {
                message_lines.push(tr!("Target (armv7hl): {}", url.bright_blue()));
            }
            if url.to_lowercase().contains("target") && url.contains("x86_64") {
                message_lines.push(tr!("Target (x86_64): {}", url.bright_blue()));
            }
            if url.to_lowercase().contains("target") && url.contains("i486") {
                message_lines.push(tr!("Target (i486): {}", url.bright_blue()));
            }
        }
        println!("{}", message_lines.join("\n"));
    }
}

impl PsdkAvailableModel {
    pub fn search() -> Vec<PsdkAvailableModel> {
        match Self::search_full() {
            Ok(value) => value,
            Err(_) => vec![],
        }
    }

    pub fn search_filter<T: Fn(&PsdkAvailableModel) -> bool>(filter: T) -> Vec<PsdkAvailableModel> {
        Self::search().iter().filter(|e| filter(e)).cloned().collect()
    }

    fn search_full() -> Result<Vec<PsdkAvailableModel>, Box<dyn std::error::Error>> {
        let url_files = utils::get_repo_url_psdk();
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
        let mut models: Vec<PsdkAvailableModel> = vec![];
        for version_full in reverse {
            let urls = version_urls.get(version_full).unwrap().clone();
            let version_major = version_full.split(".").take(3).collect::<Vec<&str>>().join(".");
            models.push(PsdkAvailableModel {
                version_major,
                version_full: version_full.to_string(),
                urls,
            });
        }
        Ok(models)
    }
}
