use crate::models::outgoing::{
    psdk_terminal::PsdkTerminalOutgoing, Outgoing, OutgoingState, OutgoingType,
};

use super::{Incoming, TraitIncoming};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PsdkTerminalIncoming {}

impl PsdkTerminalIncoming {
    pub fn new() -> Incoming {
        Incoming::PsdkTerminal(Self {})
    }
}

impl TraitIncoming for PsdkTerminalIncoming {
    fn name() -> String {
        "PsdkTerminal".into()
    }

    async fn run(&self, _: OutgoingType) -> Outgoing {
        // @todo
        PsdkTerminalOutgoing::new(OutgoingState::Error)
    }
}
