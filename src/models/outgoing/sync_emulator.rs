use serde::{Deserialize, Serialize};

use crate::utils::macros::{print_error, print_info, print_success};

use super::{Outgoing, OutgoingState, TraitOutgoing};

#[derive(Serialize, Deserialize, Clone)]
pub struct SyncEmulatorOutgoing {
    state: OutgoingState,
}

impl SyncEmulatorOutgoing {
    pub fn new(state: OutgoingState) -> Outgoing {
        Outgoing::SyncEmulator(Self { state })
    }
}

impl TraitOutgoing for SyncEmulatorOutgoing {
    fn print(&self) {
        match self.state {
            OutgoingState::Error => print_error!("произошла ошибка при синхронизации эмуляторов"),
            OutgoingState::Info => {
                print_info!("изменения конфигурации эмуляторов не зафиксировано")
            }
            OutgoingState::Success => print_success!("данные эмуляторов обновлены"),
        }
    }
}
