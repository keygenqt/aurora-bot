use serde::{Deserialize, Serialize};

use crate::models::{emulator::model::EmulatorModel, TraitModel};

use super::{Outgoing, TraitOutgoing};

#[derive(Serialize, Deserialize, Clone)]
pub struct EmulatorInfoOutgoing {
    data: Vec<EmulatorModel>,
}

impl EmulatorInfoOutgoing {
    pub fn new(data: Vec<EmulatorModel>) -> Outgoing {
        Outgoing::EmulatorInfo(Self { data })
    }
}

impl TraitOutgoing for EmulatorInfoOutgoing {
    fn print(&self) {
        <dyn TraitModel>::print_list(self.data.clone());
    }
}
