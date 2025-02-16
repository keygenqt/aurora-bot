use serde::Serialize;

use crate::models::client::incoming::TraitIncoming;
use crate::models::client::outgoing::OutgoingType;
use crate::models::client::outgoing::TraitOutgoing;
use crate::models::client::selector::incoming::SelectorIncoming;
use crate::models::client::selector::outgoing::SelectorOutgoing;
use crate::models::client::state_message::outgoing::StateMessageOutgoing;
use crate::models::TraitModel;
use crate::tools::macros::tr;

use super::model::FlutterModel;

pub struct FlutterModelSelect {}

impl FlutterModelSelect {
    pub fn select<T: TraitIncoming + Serialize + Clone>(
        key: String,
        models: Vec<FlutterModel>,
        incoming: fn(String) -> T,
    ) -> SelectorOutgoing<T> {
        SelectorOutgoing {
            key,
            variants: models
                .iter()
                .map(|e| SelectorIncoming {
                    name: tr!("Flutter SDK: {}", e.flutter_version),
                    incoming: incoming(e.get_id()),
                })
                .collect::<Vec<SelectorIncoming<T>>>(),
        }
    }

    pub fn search(id: &Option<String>, send_type: &OutgoingType) -> Vec<FlutterModel> {
        if let Some(id) = id {
            FlutterModel::search_filter(|e| e.get_id() == id.clone())
        } else {
            StateMessageOutgoing::new_state(tr!("получение Flutter SDK...")).send(send_type);
            FlutterModel::search()
        }
    }
}
