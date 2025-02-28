use std::collections::HashMap;

use dbus_crossroads::IfaceBuilder;
use human_sort::sort;
use serde::Deserialize;
use serde::Serialize;

use crate::models::client::ClientMethodsKey;
use crate::models::client::incoming::TraitIncoming;
use crate::models::client::outgoing::OutgoingType;
use crate::models::client::outgoing::TraitOutgoing;
use crate::models::client::state_message::outgoing::StateMessageOutgoing;
use crate::service::dbus::server::IfaceData;
use crate::service::responses::gitlab_tags::GitlabTagsResponse;
use crate::tools::macros::tr;
use crate::tools::single;

use super::outgoing::FlutterAvailableItemOutgoing;
use super::outgoing::FlutterAvailableOutgoing;

#[derive(Serialize, Deserialize, Clone)]
pub struct FlutterAvailableIncoming {
    is_all: bool,
}

impl FlutterAvailableIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::FlutterAvailable)
            .unwrap()
            .to_string()
    }

    pub fn new(is_all: bool) -> Box<FlutterAvailableIncoming> {
        Box::new(Self { is_all })
    }

    pub fn dbus_method_run(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            Self::name(),
            ("is_all",),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (is_all,): (bool,)| async move {
                let outgoing = Self::new(is_all).run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }
}

impl TraitIncoming for FlutterAvailableIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        StateMessageOutgoing::new_state(tr!("получение данных с репозитория")).send(&send_type);
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
        // Map to model
        let mut list: Vec<FlutterAvailableItemOutgoing> = vec![];
        for version in versions {
            let model = version_tags.get(version).unwrap();
            let created_at = match model.created_at.clone() {
                Some(value) => value,
                None => model.commit.committed_date.clone(),
            };
            list.push(FlutterAvailableItemOutgoing {
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
        if list.is_empty() {
            return StateMessageOutgoing::new_error(tr!("не удалось получить данные"));
        }
        if self.is_all {
            FlutterAvailableOutgoing::new(list)
        } else {
            let tag = list.last().unwrap().tag.clone();
            FlutterAvailableOutgoing::new(list.iter().filter(|e| e.tag == tag).cloned().collect())
        }
    }
}
