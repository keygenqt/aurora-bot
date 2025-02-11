use serde::{Deserialize, Serialize};

use crate::utils::macros::{print_error, print_success};

use super::{Outgoing, OutgoingState, TraitOutgoing};

#[derive(Serialize, Deserialize, Clone)]
pub struct PsdkTerminalOutgoing {
    state: OutgoingState,
}

impl PsdkTerminalOutgoing {
    pub fn new(state: OutgoingState) -> Outgoing {
        Outgoing::PsdkTerminal(Self { state })
    }
}

impl TraitOutgoing for PsdkTerminalOutgoing {
    fn print(&self) {
        match self.state {
            OutgoingState::Error => {
                print_error!("ошибка открытия терминала с окружением Platform SDK")
            }
            OutgoingState::Success => print_success!("терминал c окружением Platform SDK открыт"),
            _ => {}
        }
    }
}
