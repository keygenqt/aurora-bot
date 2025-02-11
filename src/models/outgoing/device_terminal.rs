use serde::{Deserialize, Serialize};

use crate::utils::macros::{print_error, print_success};

use super::{Outgoing, OutgoingState, TraitOutgoing};

#[derive(Serialize, Deserialize, Clone)]
pub struct DeviceTerminalOutgoing {
    state: OutgoingState,
}

impl DeviceTerminalOutgoing {
    pub fn new(state: OutgoingState) -> Outgoing {
        Outgoing::DeviceTerminal(Self { state })
    }
}

impl TraitOutgoing for DeviceTerminalOutgoing {
    fn print(&self) {
        match self.state {
            OutgoingState::Error => print_error!("ошибка открытия терминала для устройства"),
            OutgoingState::Success => {
                print_success!("терминал с ssh соединением устройства открыт")
            }
            _ => {}
        }
    }
}
