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

use super::model::PsdkModel;

pub struct PsdkModelSelect {}

impl PsdkModelSelect {
    pub fn select<T: TraitIncoming + Serialize + Clone>(
        key: String,
        models: Vec<PsdkModel>,
        incoming: fn(String) -> T,
    ) -> SelectorOutgoing<T> {
        SelectorOutgoing {
            key,
            variants: models
                .iter()
                .map(|e| SelectorIncoming {
                    name: tr!("Platform SDK: {}", e.version),
                    incoming: incoming(e.get_id()),
                })
                .collect::<Vec<SelectorIncoming<T>>>(),
        }
    }

    pub fn search(id: &Option<String>, send_type: &OutgoingType) -> Vec<PsdkModel> {
        if let Some(id) = id {
            PsdkModel::search_filter(|e| e.get_id() == id.clone())
        } else {
            StateMessageOutgoing::new_state(tr!("получение Platform SDK...")).send(send_type);
            PsdkModel::search()
        }
    }
}
