use crate::models::outgoing::{
    device_terminal::DeviceTerminalOutgoing, Outgoing, OutgoingState, OutgoingType,
};

use super::{Incoming, TraitIncoming};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct DeviceTerminalIncoming {}

impl DeviceTerminalIncoming {
    pub fn new() -> Incoming {
        Incoming::DeviceTerminal(Self {})
    }
}

impl TraitIncoming for DeviceTerminalIncoming {
    fn name() -> String {
        "DeviceTerminal".into()
    }

    async fn run(&self, _: OutgoingType) -> Outgoing {
        // @todo
        DeviceTerminalOutgoing::new(OutgoingState::Error)
    }
}
