use serde::{Deserialize, Serialize};

use crate::utils::macros::{print_error, print_success};

use super::{Outgoing, OutgoingState, TraitOutgoing};

#[derive(Serialize, Deserialize, Clone)]
pub struct EmulatorTerminalOutgoing {
    state: OutgoingState,
}

impl EmulatorTerminalOutgoing {
    pub fn new(state: OutgoingState) -> Outgoing {
        Outgoing::EmulatorTerminal(Self { state })
    }
}

impl TraitOutgoing for EmulatorTerminalOutgoing {
    fn print(&self) {
        match self.state {
            OutgoingState::Error => print_error!("не удалось выполнить команду: эмулятор должен быть запущен, а запуск производиться в GNOME Terminal"),
            OutgoingState::Success => print_success!("терминал с ssh соединением эмулятора открыт"),
            _ => {}
        }
    }
}
