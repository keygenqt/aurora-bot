use serde::Serialize;

use crate::{
    models::{
        client::{
            incoming::TraitIncoming,
            outgoing::{OutgoingType, TraitOutgoing},
            selector::{incoming::SelectorIncoming, outgoing::SelectorOutgoing},
            state_message::outgoing::StateMessageOutgoing,
        },
        TraitModel,
    },
    tools::macros::tr,
};

use super::model::EmulatorModel;

pub struct EmulatorModelSelect {}

impl EmulatorModelSelect {
    pub fn select<T: TraitIncoming + Serialize + Clone>(
        key: String,
        models: Vec<EmulatorModel>,
        incoming: fn(String) -> T,
    ) -> SelectorOutgoing<T> {
        SelectorOutgoing {
            key,
            variants: models
                .iter()
                .map(|e| SelectorIncoming {
                    name: tr!("Эмулятор: {}", e.uuid),
                    incoming: incoming(e.get_id()),
                })
                .collect::<Vec<SelectorIncoming<T>>>(),
        }
    }

    pub fn search(
        id: &Option<String>,
        send_type: &OutgoingType,
        is_running: bool,
    ) -> Vec<EmulatorModel> {
        if let Some(id) = id {
            EmulatorModel::search_filter(|e| e.get_id() == id.clone())
        } else {
            StateMessageOutgoing::new_state(tr!("получение эмуляторов...")).send(send_type);
            if is_running {
                EmulatorModel::search_filter(|e| e.is_running)
            } else {
                EmulatorModel::search()
            }
        }
    }
}
