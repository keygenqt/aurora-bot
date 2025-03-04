use serde::Deserialize;
use serde::Serialize;

use crate::models::TraitModel;
use crate::models::client::outgoing::DataOutgoing;
use crate::models::client::outgoing::TraitOutgoing;
use crate::models::flutter_available::model::FlutterAvailableModel;

use super::incoming::FlutterAvailableIncoming;

#[derive(Serialize, Deserialize, Clone)]
pub struct FlutterAvailableOutgoing {
    model: FlutterAvailableModel,
}

impl FlutterAvailableOutgoing {
    pub fn new(model: FlutterAvailableModel) -> Box<FlutterAvailableOutgoing> {
        Box::new(Self { model })
    }
}

impl TraitOutgoing for FlutterAvailableOutgoing {
    fn print(&self) {
        self.model.print();
    }

    fn to_json(&self) -> String {
        DataOutgoing::serialize(FlutterAvailableIncoming::name(), self.clone())
    }
}
