use color_eyre::owo_colors::OwoColorize;
use serde::Serialize;

use crate::models::TraitModel;
use crate::models::client::incoming::TraitIncoming;
use crate::models::client::outgoing::OutgoingType;
use crate::models::client::outgoing::TraitOutgoing;
use crate::models::client::selector::outgoing::incoming::SelectorIncoming;
use crate::models::client::selector::outgoing::outgoing::SelectorOutgoing;
use crate::models::client::state_message::outgoing::StateMessageOutgoing;
use crate::models::demo_app::model::DemoAppModel;
use crate::tools::macros::tr;

pub struct DemoAppModelSelect {}

impl DemoAppModelSelect {
    pub fn select<T: TraitIncoming + Serialize + Clone, F: Fn(String) -> T>(
        key: String,
        models: Vec<DemoAppModel>,
        incoming: F,
    ) -> Result<SelectorOutgoing<T>, Box<dyn std::error::Error>> {
        Ok(SelectorOutgoing {
            key,
            variants: models
                .iter()
                .map(|e| SelectorIncoming {
                    name: tr!("{}. {}", e.get_key().bright_white().bold(), e.desc),
                    incoming: incoming(e.get_id()),
                })
                .collect::<Vec<SelectorIncoming<T>>>(),
        })
    }

    pub fn search(id: &Option<String>, text: String, send_type: &OutgoingType) -> Vec<DemoAppModel> {
        if let Some(id) = id {
            DemoAppModel::search_filter(|e| e.get_id() == id.clone())
        } else {
            StateMessageOutgoing::new_state(text).send(send_type);
            DemoAppModel::search()
        }
    }
}
