use super::{Incoming, TraitIncoming};
use crate::models::{
    configuration::{flutter::FlutterConfig, Config},
    outgoing::{
        sync_flutter::SyncFlutterOutgoing, sync_start_state::SyncStartStateOutgoing, Outgoing,
        OutgoingState, OutgoingType,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct SyncFlutterIncoming {}

impl SyncFlutterIncoming {
    pub fn new() -> Incoming {
        Incoming::SyncFlutter(Self {})
    }
}

impl TraitIncoming for SyncFlutterIncoming {
    fn name() -> String {
        "SyncFlutter".into()
    }

    async fn run(&self, send_type: OutgoingType) -> Outgoing {
        // Send state
        SyncStartStateOutgoing::new_flutter().send(&send_type);
        // Run sync
        if Config::save_flutter(FlutterConfig::search().await) {
            SyncFlutterOutgoing::new(OutgoingState::Success)
        } else {
            SyncFlutterOutgoing::new(OutgoingState::Info)
        }
    }
}
