use serde::Serialize;

use crate::feature::incoming::TraitIncoming;
use crate::feature::outgoing::OutgoingType;
use crate::feature::outgoing::TraitOutgoing;
use crate::feature::selector::outgoing::incoming::SelectorIncoming;
use crate::feature::selector::outgoing::outgoing::SelectorOutgoing;
use crate::feature::state_message::outgoing::StateMessageOutgoing;
use crate::models::TraitModel;
use crate::models::device::model::DeviceModel;
use crate::tools::macros::tr;

pub struct DeviceModelSelect {}

#[allow(dead_code)]
impl DeviceModelSelect {
    pub fn select<T: TraitIncoming + Serialize + Clone, F: Fn(String) -> T>(
        key: String,
        send_type: &OutgoingType,
        models: Vec<DeviceModel>,
        incoming: F,
    ) -> Result<SelectorOutgoing<T>, Box<dyn std::error::Error>> {
        Ok(SelectorOutgoing {
            key,
            send_type: send_type.clone(),
            variants: models
                .iter()
                .map(|e| SelectorIncoming {
                    name: tr!("Устройство: {}", e.get_key()),
                    incoming: incoming(e.get_id()),
                })
                .collect::<Vec<SelectorIncoming<T>>>(),
        })
    }

    pub fn search(id: &Option<String>, text: String, send_type: &OutgoingType) -> Vec<DeviceModel> {
        if let Some(id) = id {
            DeviceModel::search_filter(|e| e.get_id() == id.clone())
        } else {
            StateMessageOutgoing::new_state(text).send(send_type);
            DeviceModel::search()
        }
    }
}
