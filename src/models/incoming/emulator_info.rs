use crate::models::{
    emulator::model::EmulatorModel,
    outgoing::{emulator_info::EmulatorInfoOutgoing, Outgoing, OutgoingType},
};

use super::{Incoming, TraitIncoming};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct EmulatorInfoIncoming {}

impl EmulatorInfoIncoming {
    pub fn new() -> Incoming {
        Incoming::EmulatorInfo(Self {})
    }
}

impl TraitIncoming for EmulatorInfoIncoming {
    fn name() -> String {
        "EmulatorInfo".into()
    }

    async fn run(&self, _: OutgoingType) -> Outgoing {
        EmulatorInfoOutgoing::new(EmulatorModel::search().await)
    }
}
