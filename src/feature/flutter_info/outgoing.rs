use serde::Deserialize;
use serde::Serialize;

use crate::feature::outgoing::DataOutgoing;
use crate::feature::outgoing::TraitOutgoing;
use crate::models::TraitModel;
use crate::models::flutter_installed::model::FlutterInstalledModel;

use super::incoming::FlutterInfoIncoming;

#[derive(Serialize, Deserialize, Clone)]
pub struct FlutterInfoOutgoing {
    model: FlutterInstalledModel,
}

impl FlutterInfoOutgoing {
    pub fn new(model: FlutterInstalledModel) -> Box<FlutterInfoOutgoing> {
        Box::new(Self { model })
    }
}

impl TraitOutgoing for FlutterInfoOutgoing {
    fn print(&self) {
        self.model.print();
    }

    fn to_json(&self) -> String {
        DataOutgoing::serialize(FlutterInfoIncoming::name(), self.clone())
    }
}
