use serde::{Deserialize, Serialize};

use crate::utils::macros::{print_error, print_success};

use super::{Outgoing, OutgoingState, TraitOutgoing};

#[derive(Serialize, Deserialize, Clone)]
pub struct FlutterTerminalOutgoing {
    state: OutgoingState,
}

impl FlutterTerminalOutgoing {
    pub fn new(state: OutgoingState) -> Outgoing {
        Outgoing::FlutterTerminal(Self { state })
    }
}

impl TraitOutgoing for FlutterTerminalOutgoing {
    fn print(&self) {
        match self.state {
            OutgoingState::Error => print_error!("ошибка открытия терминала с окружением Flutter"),
            OutgoingState::Success => print_success!("терминал c окружением Flutter открыт"),
            _ => {}
        }
    }
}
