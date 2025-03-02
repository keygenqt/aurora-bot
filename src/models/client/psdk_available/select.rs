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

use super::outgoing::PsdkAvailableItemOutgoing;

pub struct PsdkAvailableSelect {}

impl PsdkAvailableSelect {
    pub fn select<T: TraitIncoming + Serialize + Clone, F: Fn(String) -> T>(
        key: String,
        models: Vec<PsdkAvailableItemOutgoing>,
        incoming: F,
    ) -> SelectorOutgoing<T> {
        SelectorOutgoing {
            key,
            variants: models
                .iter()
                .map(|e| SelectorIncoming {
                    name: tr!("Platform SDK: {}", e.version_full),
                    incoming: incoming(e.version_full.clone()),
                })
                .collect::<Vec<SelectorIncoming<T>>>(),
        }
    }

    pub fn search(id: &Option<String>, send_type: &OutgoingType, text_select: String, text_model: String) -> Vec<PsdkAvailableItemOutgoing> {
        if id.is_none() {
            StateMessageOutgoing::new_state(text_select).send(send_type);
        } else {
            StateMessageOutgoing::new_state(text_model).send(send_type);
        }
        let url_files = single::get_request().get_repo_url_psdk();
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
        let mut models: Vec<PsdkAvailableItemOutgoing> = vec![];
        for version_full in reverse {
            let urls = version_urls.get(version_full).unwrap().clone();
            let version_major = version_full.split(".").take(3).collect::<Vec<&str>>().join(".");
            models.push(PsdkAvailableItemOutgoing {
                version_major,
                version_full: version_full.to_string(),
                urls,
            });
        }
        if let Some(id) = id {
            models.iter().filter(|e| e.version_full == id.clone()).cloned().collect()
        } else {
            models
        }
    }
}
