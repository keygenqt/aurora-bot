use super::{Incoming, TraitIncoming};
use crate::models::{
    configuration::{sdk::SdkConfig, Config},
    outgoing::{
        sync_sdk::SyncSdkOutgoing, sync_start_state::SyncStartStateOutgoing, Outgoing,
        OutgoingState, OutgoingType,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct SyncSdkIncoming {}

impl SyncSdkIncoming {
    pub fn new() -> Incoming {
        Incoming::SyncSdk(Self {})
    }
}

impl TraitIncoming for SyncSdkIncoming {
    fn name() -> String {
        "SyncSdk".into()
    }

    async fn run(&self, send_type: OutgoingType) -> Outgoing {
        // Send state
        SyncStartStateOutgoing::new_sdk().send(&send_type);
        // Run sync
        if Config::save_sdk(SdkConfig::search().await) {
            SyncSdkOutgoing::new(OutgoingState::Success)
        } else {
            SyncSdkOutgoing::new(OutgoingState::Info)
        }
    }
}
