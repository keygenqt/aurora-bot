use serde::Deserialize;
use serde::Serialize;

use crate::models::client::outgoing::DataOutgoing;
use crate::models::client::outgoing::TraitOutgoing;

use super::incoming::SdkAvailableIncoming;

#[derive(Serialize, Deserialize, Clone)]
pub struct SdkAvailableOutgoing {
    models: Vec<SdkAvailableItemOutgoing>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SdkAvailableItemOutgoing {
    name: String,
    url: String,
}

impl SdkAvailableOutgoing {
    pub fn new(models: Vec<SdkAvailableItemOutgoing>) -> Box<SdkAvailableOutgoing> {
        Box::new(Self { models })
    }
}

impl TraitOutgoing for SdkAvailableOutgoing {
    fn print(&self) {
        println!("@todo sdk available")
    }

    fn to_json(&self) -> String {
        DataOutgoing::serialize(SdkAvailableIncoming::name(), self.clone())
    }
}
