use serde::Deserialize;
use serde::Serialize;

use crate::models::client::outgoing::DataOutgoing;
use crate::models::client::outgoing::TraitOutgoing;

use super::incoming::PsdkAvailableIncoming;

#[derive(Serialize, Deserialize, Clone)]
pub struct PsdkAvailableOutgoing {
    models: Vec<PsdkAvailableItemOutgoing>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PsdkAvailableItemOutgoing {
    name: String,
    urls: Vec<String>,
}

impl PsdkAvailableOutgoing {
    pub fn new(models: Vec<PsdkAvailableItemOutgoing>) -> Box<PsdkAvailableOutgoing> {
        Box::new(Self { models })
    }
}

impl TraitOutgoing for PsdkAvailableOutgoing {
    fn print(&self) {
        println!("@todo psdk available")
    }

    fn to_json(&self) -> String {
        DataOutgoing::serialize(PsdkAvailableIncoming::name(), self.clone())
    }
}
