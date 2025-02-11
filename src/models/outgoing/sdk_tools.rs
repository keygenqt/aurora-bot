use serde::{Deserialize, Serialize};

use crate::utils::macros::{print_error, print_success};

use super::{Outgoing, OutgoingState, TraitOutgoing};

#[derive(Serialize, Deserialize, Clone)]
pub struct SdkToolsOutgoing {
    state: OutgoingState,
}

impl SdkToolsOutgoing {
    pub fn new(state: OutgoingState) -> Outgoing {
        Outgoing::SdkTools(Self { state })
    }
}

impl TraitOutgoing for SdkToolsOutgoing {
    fn print(&self) {
        match self.state {
            OutgoingState::Error => print_error!("ошибка открытия SDK maintenance tools"),
            OutgoingState::Success => print_success!("SDK maintenance tools открыт"),
            _ => {}
        }
    }
}
