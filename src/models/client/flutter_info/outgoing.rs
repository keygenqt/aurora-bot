use serde::{Deserialize, Serialize};

use crate::models::{
    client::outgoing::{DataOutgoing, TraitOutgoing},
    flutter::model::FlutterModel,
    TraitModel,
};

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
