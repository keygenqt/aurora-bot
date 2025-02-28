use std::collections::HashMap;

use dbus_crossroads::IfaceBuilder;
use human_sort::sort;
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;

use crate::models::client::ClientMethodsKey;
use crate::models::client::incoming::TraitIncoming;
use crate::models::client::outgoing::OutgoingType;
use crate::models::client::outgoing::TraitOutgoing;
use crate::models::client::state_message::outgoing::StateMessageOutgoing;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::tr;
use crate::tools::single;

use super::outgoing::PsdkAvailableItemOutgoing;
use super::outgoing::PsdkAvailableOutgoing;

#[derive(Serialize, Deserialize, Clone)]
pub struct PsdkAvailableIncoming {
    is_all: bool,
}

impl PsdkAvailableIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::PsdkAvailable)
            .unwrap()
            .to_string()
    }

    pub fn new(is_all: bool) -> Box<PsdkAvailableIncoming> {
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

impl TraitIncoming for PsdkAvailableIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        StateMessageOutgoing::new_state(tr!("получение данных с репозитория")).send(&send_type);
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
        // Map to model
        let mut list: Vec<PsdkAvailableItemOutgoing> = vec![];
        for version_full in versions {
            let urls = version_urls.get(version_full).unwrap().clone();
            let version_major = version_full.split(".").take(3).collect::<Vec<&str>>().join(".");
            list.push(PsdkAvailableItemOutgoing {
                version_major,
                version_full: version_full.to_string(),
                urls,
            });
        }
        if list.is_empty() {
            return StateMessageOutgoing::new_error(tr!("не удалось получить данные"));
        }
        if self.is_all {
            PsdkAvailableOutgoing::new(list)
        } else {
            let version_last = list.last().unwrap().version_full.clone();
            PsdkAvailableOutgoing::new(
                list.iter()
                    .filter(|e| e.version_full == version_last)
                    .cloned()
                    .collect(),
            )
        }
    }
}
