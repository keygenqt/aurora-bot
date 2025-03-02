use std::collections::HashMap;

use human_sort::sort;
use regex::Regex;
use serde::Serialize;

use crate::models::client::incoming::TraitIncoming;
use crate::models::client::outgoing::OutgoingType;
use crate::models::client::outgoing::TraitOutgoing;
use crate::models::client::selector::outgoing::incoming::SelectorIncoming;
use crate::models::client::selector::outgoing::outgoing::SelectorOutgoing;
use crate::models::client::state_message::outgoing::StateMessageOutgoing;
use crate::tools::macros::tr;
use crate::tools::single;

use super::outgoing::SdkAvailableItemOutgoing;
use super::outgoing::SdkBuildType;
use super::outgoing::SdkInstallType;

pub struct SdkAvailableSelect {}

impl SdkAvailableSelect {
    pub fn select<T: TraitIncoming + Serialize + Clone, F: Fn(String) -> T>(
        key: String,
        models: Vec<SdkAvailableItemOutgoing>,
        incoming: F,
    ) -> SelectorOutgoing<T> {
        SelectorOutgoing {
            key,
            variants: models
                .iter()
                .map(|e| SelectorIncoming {
                    name: tr!("Аврора SDK: {} ({}, {})", e.version_full, e.name_build_type(), e.name_install_type()),
                    incoming: incoming(format!("{}:{}:{}", e.version_full, e.name_build_type(), e.name_install_type())),
                })
                .collect::<Vec<SelectorIncoming<T>>>(),
        }
    }

    pub fn search(id: &Option<String>, send_type: &OutgoingType, text_select: String, text_model: String) -> Vec<SdkAvailableItemOutgoing> {
        if id.is_none() {
            StateMessageOutgoing::new_state(text_select).send(send_type);
        } else {
            StateMessageOutgoing::new_state(text_model).send(send_type);
        }
        let url_files = single::get_request().get_repo_url_sdk();
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
        let mut models: Vec<SdkAvailableItemOutgoing> = vec![];
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
                models.push(SdkAvailableItemOutgoing {
                    url,
                    version_major,
                    version_full,
                    build_type,
                    install_type,
                });
            }
        }
        if let Some(id) = id {
            let keys: Vec<&str> = id.split(":").collect();
            let versions: Vec<SdkAvailableItemOutgoing> = models.iter().filter(|e| e.version_full == keys.get(0).unwrap().to_string()).cloned().collect();
            let builds: Vec<SdkAvailableItemOutgoing> = versions.iter().filter(|e| e.name_build_type() == keys.get(1).unwrap().to_string()).cloned().collect();
            let install: Vec<SdkAvailableItemOutgoing> = builds.iter().filter(|e| e.name_install_type() == keys.get(2).unwrap().to_string()).cloned().collect();
            install
        } else {
            models
        }
    }
}
