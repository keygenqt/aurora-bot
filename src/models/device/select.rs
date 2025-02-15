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

use super::model::DeviceModel;

#[allow(dead_code)]
pub struct DeviceModelSelect {}

#[allow(dead_code)]
impl DeviceModelSelect {
    pub fn select<T: TraitIncoming + Serialize + Clone>(
        key: String,
        models: Vec<DeviceModel>,
        incoming: fn(String) -> T,
    ) -> SelectorOutgoing<T> {
        SelectorOutgoing {
            key,
            variants: models
                .iter()
                .map(|e| SelectorIncoming {
                    name: tr!("Устройство: {}", e.ip),
                    incoming: incoming(e.get_id()),
                })
                .collect::<Vec<SelectorIncoming<T>>>(),
        }
    }

    pub fn search(id: &Option<String>, send_type: &OutgoingType) -> Vec<DeviceModel> {
        if let Some(id) = id {
            DeviceModel::search_filter(|e| e.get_id() == id.clone())
        } else {
            StateMessageOutgoing::new_state(tr!("получение устройств...")).send(send_type);
            DeviceModel::search()
        }
    }
}
