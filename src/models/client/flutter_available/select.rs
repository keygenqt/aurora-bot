use std::collections::HashMap;

use human_sort::sort;
use serde::Serialize;

use crate::models::client::incoming::TraitIncoming;
use crate::models::client::outgoing::OutgoingType;
use crate::models::client::outgoing::TraitOutgoing;
use crate::models::client::selector::outgoing::incoming::SelectorIncoming;
use crate::models::client::selector::outgoing::outgoing::SelectorOutgoing;
use crate::models::client::state_message::outgoing::StateMessageOutgoing;
use crate::service::responses::gitlab_tags::GitlabTagsResponse;
use crate::tools::macros::tr;
use crate::tools::single;

use super::outgoing::FlutterAvailableItemOutgoing;

pub struct FlutterAvailableSelect {}

impl FlutterAvailableSelect {
    pub fn select<T: TraitIncoming + Serialize + Clone, F: Fn(String) -> T>(
        key: String,
        models: Vec<FlutterAvailableItemOutgoing>,
        incoming: F,
    ) -> SelectorOutgoing<T> {
        SelectorOutgoing {
            key,
            variants: models
                .iter()
                .map(|e| SelectorIncoming {
                    name: tr!("Flutter SDK: {}", e.version),
                    incoming: incoming(e.get_id()),
                })
                .collect::<Vec<SelectorIncoming<T>>>(),
        }
    }

    pub fn search(
        id: &Option<String>,
        send_type: &OutgoingType,
        text_select: String,
        text_model: String,
    ) -> Vec<FlutterAvailableItemOutgoing> {
        if id.is_none() {
            StateMessageOutgoing::new_state(text_select).send(send_type);
        } else {
            StateMessageOutgoing::new_state(text_model).send(send_type);
        }
        let tags_flutter = single::get_request().get_repo_tags_flutter();
        // Clear tags version
        let mut versions: Vec<String> = vec![];
        let mut version_tags: HashMap<String, GitlabTagsResponse> = HashMap::new();
        for tag in tags_flutter {
            let version = tag.name.replace("aurora", "").trim_matches('-').to_string();
            if !version_tags.contains_key(&version) {
                version_tags.insert(version.clone(), tag);
                versions.push(version);
            }
        }
        // Sort version
        let mut versions = versions.iter().map(|e| e.as_str()).collect::<Vec<&str>>();
        sort(&mut versions);
        let reverse = versions.iter().copied().rev().collect::<Vec<&str>>();
        // Map to model
        let mut models: Vec<FlutterAvailableItemOutgoing> = vec![];
        for version in reverse {
            let model = version_tags.get(version).unwrap();
            let created_at = match model.created_at.clone() {
                Some(value) => value,
                None => model.commit.committed_date.clone(),
            };
            models.push(FlutterAvailableItemOutgoing {
                tag: model.name.clone(),
                version: version.to_string(),
                created_at,
                url_gitlab: format!("https://gitlab.com/omprussia/flutter/flutter/-/tree/{version}"),
                url_zip: format!(
                    "https://gitlab.com/omprussia/flutter/flutter/-/archive/{version}/flutter-{version}.zip"
                ),
                url_tar_gz: format!(
                    "https://gitlab.com/omprussia/flutter/flutter/-/archive/{version}/flutter-{version}.tar.gz"
                ),
            });
        }
        if let Some(id) = id {
            models.iter().filter(|e| e.get_id() == id.clone()).cloned().collect()
        } else {
            models
        }
    }
}
