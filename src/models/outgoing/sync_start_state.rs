use serde::{Deserialize, Serialize};

use crate::utils::macros::print_state;

use super::{Outgoing, TraitOutgoing};

#[derive(Deserialize, Serialize, Clone)]
pub enum SyncStartState {
    Device,
    Emulator,
    Flutter,
    Psdk,
    Sdk,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SyncStartStateOutgoing {
    pub state: SyncStartState,
}

impl SyncStartStateOutgoing {
    pub fn new_device() -> Outgoing {
        Outgoing::SyncStartState(Self {
            state: SyncStartState::Device,
        })
    }

    pub fn new_emulator() -> Outgoing {
        Outgoing::SyncStartState(Self {
            state: SyncStartState::Emulator,
        })
    }

    pub fn new_flutter() -> Outgoing {
        Outgoing::SyncStartState(Self {
            state: SyncStartState::Flutter,
        })
    }

    pub fn new_psdk() -> Outgoing {
        Outgoing::SyncStartState(Self {
            state: SyncStartState::Psdk,
        })
    }

    pub fn new_sdk() -> Outgoing {
        Outgoing::SyncStartState(Self {
            state: SyncStartState::Sdk,
        })
    }
}

impl TraitOutgoing for SyncStartStateOutgoing {
    fn print(&self) {
        match self.state {
            SyncStartState::Device => print_state!("синхронизация устройств..."),
            SyncStartState::Emulator => print_state!("синхронизация эмуляторов..."),
            SyncStartState::Flutter => print_state!("синхронизация Flutter SDK..."),
            SyncStartState::Psdk => print_state!("синхронизация Platform SDK..."),
            SyncStartState::Sdk => print_state!("синхронизация Аврора SDK..."),
        }
    }
}
