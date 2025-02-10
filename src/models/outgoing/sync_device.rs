use serde::{Deserialize, Serialize};

use crate::utils::macros::{print_error, print_info, print_success};

use super::{Outgoing, OutgoingState, TraitOutgoing};

#[derive(Serialize, Deserialize, Clone)]
pub struct SyncDeviceOutgoing {
    state: OutgoingState,
}

impl SyncDeviceOutgoing {
    pub fn new(state: OutgoingState) -> Outgoing {
        Outgoing::SyncDevice(Self { state })
    }
}

impl TraitOutgoing for SyncDeviceOutgoing {
    fn print(&self) {
        match self.state {
            OutgoingState::Error => print_error!("произошла ошибка при синхронизации устройств"),
            OutgoingState::Info => print_info!("изменения конфигурации устройств не зафиксировано"),
            OutgoingState::Success => print_success!("данные устройств обновлены"),
        }
    }
}
