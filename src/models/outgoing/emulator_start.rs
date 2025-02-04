use serde::{Deserialize, Serialize};

use crate::utils::macros::{print_error, print_info, print_success};

use super::{Outgoing, OutgoingState, TraitOutgoing};

#[derive(Serialize, Deserialize, Clone)]
pub struct OutgoingEmulatorStart {
    state: OutgoingState,
    os_name: String,
}

impl OutgoingEmulatorStart {
    pub fn new(state: OutgoingState, os_name: String) -> Outgoing {
        Outgoing::EmulatorStart(Self { state, os_name })
    }
}

impl TraitOutgoing for OutgoingEmulatorStart {
    fn print(&self) {
        match self.state {
            OutgoingState::Error => {
                let message = format!("не удалось запустить эмулятор");
                print_error!(message)
            }
            OutgoingState::Info => {
                let message = format!("эмулятор уже запущен: {}", self.os_name);
                print_info!(message)
            }
            OutgoingState::Success => {
                let message = format!("эмулятор успешно запущен: {}", self.os_name);
                print_success!(message)
            }
        }
    }
}
