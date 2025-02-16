use serde::Deserialize;
use serde::Serialize;

use crate::models::client::outgoing::DataOutgoing;
use crate::models::client::outgoing::TraitOutgoing;
use crate::models::flutter::model::FlutterModel;
use crate::models::TraitModel;

use super::incoming::FlutterInfoIncoming;

#[derive(Serialize, Deserialize, Clone)]
pub struct FlutterInfoOutgoing {
    model: FlutterModel,
}

impl FlutterInfoOutgoing {
    pub fn new(model: FlutterModel) -> Box<FlutterInfoOutgoing> {
        Box::new(Self { model })
    }
}

impl TraitOutgoing for FlutterInfoOutgoing {
    fn print(&self) {
        self.model.print();
    }

    fn to_string(&self) -> String {
        DataOutgoing::serialize(FlutterInfoIncoming::name(), self.clone())
    }
}
