use serde::Serialize;

use crate::models::TraitModel;
use crate::models::client::incoming::TraitIncoming;
use crate::models::client::outgoing::OutgoingType;
use crate::models::client::outgoing::TraitOutgoing;
use crate::models::client::selector::outgoing::incoming::SelectorIncoming;
use crate::models::client::selector::outgoing::outgoing::SelectorOutgoing;
use crate::models::client::state_message::outgoing::StateMessageOutgoing;
use crate::models::sdk_available::model::SdkAvailableModel;
use crate::tools::macros::tr;

pub struct SdkAvailableModelSelect {}

impl SdkAvailableModelSelect {
    pub fn select<T: TraitIncoming + Serialize + Clone, F: Fn(String) -> T>(
        key: String,
        models: Vec<SdkAvailableModel>,
        incoming: F,
    ) -> Result<SelectorOutgoing<T>, Box<dyn std::error::Error>> {
        Ok(SelectorOutgoing {
            key,
            variants: models
                .iter()
                .map(|e| SelectorIncoming {
                    name: tr!("Аврора SDK: {}", e.get_key()),
                    incoming: incoming(e.get_id()),
                })
                .collect::<Vec<SelectorIncoming<T>>>(),
        })
    }

    pub fn search(id: &Option<String>, text: String, send_type: &OutgoingType) -> Vec<SdkAvailableModel> {
        if let Some(id) = id {
            SdkAvailableModel::search_filter(|e| e.get_id() == id.clone())
        } else {
            StateMessageOutgoing::new_state(text).send(send_type);
            SdkAvailableModel::search()
        }
    }
}
