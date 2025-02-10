use super::{Incoming, TraitIncoming};
use crate::models::{
    configuration::{emulator::EmulatorConfig, Config},
    outgoing::{
        sync_emulator::SyncEmulatorOutgoing, sync_start_state::SyncStartStateOutgoing, Outgoing,
        OutgoingState, OutgoingType,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct SyncEmulatorIncoming {}

impl SyncEmulatorIncoming {
    pub fn new() -> Incoming {
        Incoming::SyncEmulator(Self {})
    }
}

impl TraitIncoming for SyncEmulatorIncoming {
    fn name() -> String {
        "SyncEmulator".into()
    }

    async fn run(&self, send_type: OutgoingType) -> Outgoing {
        // Send state
        SyncStartStateOutgoing::new_emulator().send(&send_type);
        // Run sync
        if Config::save_emulator(EmulatorConfig::search().await) {
            SyncEmulatorOutgoing::new(OutgoingState::Success)
        } else {
            SyncEmulatorOutgoing::new(OutgoingState::Info)
        }
    }
}
