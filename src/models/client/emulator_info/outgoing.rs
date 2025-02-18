use serde::Deserialize;
use serde::Serialize;

use crate::models::client::outgoing::DataOutgoing;
use crate::models::client::outgoing::TraitOutgoing;
use crate::models::emulator::model::EmulatorModel;
use crate::models::TraitModel;

use super::incoming::EmulatorInfoIncoming;

#[derive(Serialize, Deserialize, Clone)]
pub struct EmulatorInfoOutgoing {
    model: EmulatorModel,
}

impl EmulatorInfoOutgoing {
    pub fn new(model: EmulatorModel) -> Box<EmulatorInfoOutgoing> {
        Box::new(Self { model })
    }
}

impl TraitOutgoing for EmulatorInfoOutgoing {
    fn print(&self) {
        self.model.print();
    }

    fn to_json(&self) -> String {
        DataOutgoing::serialize(EmulatorInfoIncoming::name(), self.clone())
    }
}
