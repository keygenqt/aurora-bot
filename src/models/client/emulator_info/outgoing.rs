use serde::{Deserialize, Serialize};

use crate::models::{
    client::outgoing::{DataOutgoing, TraitOutgoing},
    emulator::model::EmulatorModel,
    TraitModel,
};

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

    fn to_string(&self) -> String {
        DataOutgoing::serialize(EmulatorInfoIncoming::name(), self.clone())
    }
}
