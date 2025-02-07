use serde::{Deserialize, Serialize};

use crate::utils::macros::{print_error, print_info, print_success};

use super::{Outgoing, OutgoingState, TraitOutgoing};

#[derive(Serialize, Deserialize, Clone)]
pub struct EmulatorStartOutgoing {
    state: OutgoingState,
    os_name: String,
}

impl EmulatorStartOutgoing {
    pub fn new(state: OutgoingState, os_name: String) -> Outgoing {
        Outgoing::EmulatorStart(Self { state, os_name })
    }
}

impl TraitOutgoing for EmulatorStartOutgoing {
    fn print(&self) {
        match self.state {
            OutgoingState::Error => {
                let message = "не удалось запустить эмулятор".to_string();
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
