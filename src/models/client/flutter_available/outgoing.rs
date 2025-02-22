use serde::Deserialize;
use serde::Serialize;

use crate::models::client::outgoing::DataOutgoing;
use crate::models::client::outgoing::TraitOutgoing;

use super::incoming::FlutterAvailableIncoming;

#[derive(Serialize, Deserialize, Clone)]
pub struct FlutterAvailableOutgoing {
    models: Vec<FlutterAvailableItemOutgoing>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FlutterAvailableItemOutgoing {
    name: String,
    git: String,
}

impl FlutterAvailableOutgoing {
    pub fn new(models: Vec<FlutterAvailableItemOutgoing>) -> Box<FlutterAvailableOutgoing> {
        Box::new(Self { models })
    }
}

impl TraitOutgoing for FlutterAvailableOutgoing {
    fn print(&self) {
        println!("@todo flutter available")
    }

    fn to_json(&self) -> String {
        DataOutgoing::serialize(FlutterAvailableIncoming::name(), self.clone())
    }
}
