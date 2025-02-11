use crate::models::outgoing::{
    flutter_terminal::FlutterTerminalOutgoing, Outgoing, OutgoingState, OutgoingType,
};

use super::{Incoming, TraitIncoming};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct FlutterTerminalIncoming {}

impl FlutterTerminalIncoming {
    pub fn new() -> Incoming {
        Incoming::FlutterTerminal(Self {})
    }
}

impl TraitIncoming for FlutterTerminalIncoming {
    fn name() -> String {
        "FlutterTerminal".into()
    }

    async fn run(&self, _: OutgoingType) -> Outgoing {
        // @todo
        FlutterTerminalOutgoing::new(OutgoingState::Error)
    }
}
