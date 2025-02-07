use serde::{Deserialize, Serialize};

use crate::utils::macros::{print_error, print_info, print_success};

use super::{Outgoing, OutgoingState, TraitOutgoing};

#[derive(Serialize, Deserialize, Clone)]
pub struct EmulatorCloseOutgoing {
    state: OutgoingState,
}

impl EmulatorCloseOutgoing {
    pub fn new(state: OutgoingState) -> Outgoing {
        Outgoing::EmulatorClose(Self { state })
    }
}

impl TraitOutgoing for EmulatorCloseOutgoing {
    fn print(&self) {
        match self.state {
            OutgoingState::Error => print_error!("при закрытии эмулятора произошла ошибка"),
            OutgoingState::Info => print_info!("запущенные эмуляторы не найдены"),
            OutgoingState::Success => print_success!("все эмуляторы закрыты"),
        }
    }
}
