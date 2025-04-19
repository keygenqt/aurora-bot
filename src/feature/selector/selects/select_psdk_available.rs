use serde::Serialize;

use crate::feature::incoming::TraitIncoming;
use crate::feature::outgoing::OutgoingType;
use crate::feature::outgoing::TraitOutgoing;
use crate::feature::selector::outgoing::incoming::SelectorIncoming;
use crate::feature::selector::outgoing::outgoing::SelectorOutgoing;
use crate::feature::state_message::outgoing::StateMessageOutgoing;
use crate::models::TraitModel;
use crate::models::psdk_available::model::PsdkAvailableModel;
use crate::tools::macros::tr;

pub struct PsdkAvailableModelSelect {}

impl PsdkAvailableModelSelect {
    pub fn select<T: TraitIncoming + Serialize + Clone, F: Fn(String) -> T>(
        key: String,
        send_type: &OutgoingType,
        models: Vec<PsdkAvailableModel>,
        incoming: F,
    ) -> Result<SelectorOutgoing<T>, Box<dyn std::error::Error>> {
        Ok(SelectorOutgoing {
            key,
            send_type: send_type.clone(),
            variants: models
                .iter()
                .map(|e| SelectorIncoming {
                    name: tr!("Platform SDK: {}", e.get_key()),
                    incoming: incoming(e.get_id()),
                })
                .collect::<Vec<SelectorIncoming<T>>>(),
        })
    }

    pub fn search(id: &Option<String>, text: String, send_type: &OutgoingType) -> Vec<PsdkAvailableModel> {
        if let Some(id) = id {
            PsdkAvailableModel::search_filter(|e| e.get_id() == id.clone())
        } else {
            StateMessageOutgoing::new_state(text).send(send_type);
            PsdkAvailableModel::search()
        }
    }
}
