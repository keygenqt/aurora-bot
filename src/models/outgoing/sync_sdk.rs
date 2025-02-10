use serde::{Deserialize, Serialize};

use crate::utils::macros::{print_error, print_info, print_success};

use super::{Outgoing, OutgoingState, TraitOutgoing};

#[derive(Serialize, Deserialize, Clone)]
pub struct SyncSdkOutgoing {
    state: OutgoingState,
}

impl SyncSdkOutgoing {
    pub fn new(state: OutgoingState) -> Outgoing {
        Outgoing::SyncSdk(Self { state })
    }
}

impl TraitOutgoing for SyncSdkOutgoing {
    fn print(&self) {
        match self.state {
            OutgoingState::Error => print_error!("произошла ошибка при синхронизации Аврора SDK"),
            OutgoingState::Info => {
                print_info!("изменения конфигурации Аврора SDK не зафиксировано")
            }
            OutgoingState::Success => print_success!("данные Аврора SDK обновлены"),
        }
    }
}
