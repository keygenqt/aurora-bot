use serde::{Deserialize, Serialize};

use crate::utils::macros::{print_error, print_info, print_success};

use super::{Outgoing, OutgoingState, TraitOutgoing};

#[derive(Serialize, Deserialize, Clone)]
pub struct SyncPsdkOutgoing {
    state: OutgoingState,
}

impl SyncPsdkOutgoing {
    pub fn new(state: OutgoingState) -> Outgoing {
        Outgoing::SyncPsdk(Self { state })
    }
}

impl TraitOutgoing for SyncPsdkOutgoing {
    fn print(&self) {
        match self.state {
            OutgoingState::Error => print_error!("произошла ошибка при синхронизации Platform SDK"),
            OutgoingState::Info => {
                print_info!("изменения конфигурации Platform SDK не зафиксировано")
            }
            OutgoingState::Success => print_success!("данные Platform SDK обновлены"),
        }
    }
}
