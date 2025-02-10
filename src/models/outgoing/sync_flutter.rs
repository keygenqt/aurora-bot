use serde::{Deserialize, Serialize};

use crate::utils::macros::{print_error, print_info, print_success};

use super::{Outgoing, OutgoingState, TraitOutgoing};

#[derive(Serialize, Deserialize, Clone)]
pub struct SyncFlutterOutgoing {
    state: OutgoingState,
}

impl SyncFlutterOutgoing {
    pub fn new(state: OutgoingState) -> Outgoing {
        Outgoing::SyncFlutter(Self { state })
    }
}

impl TraitOutgoing for SyncFlutterOutgoing {
    fn print(&self) {
        match self.state {
            OutgoingState::Error => print_error!("произошла ошибка при синхронизации Flutter SDK"),
            OutgoingState::Info => {
                print_info!("изменения конфигурации Flutter SDK не зафиксировано")
            }
            OutgoingState::Success => print_success!("данные Flutter SDK обновлены"),
        }
    }
}
