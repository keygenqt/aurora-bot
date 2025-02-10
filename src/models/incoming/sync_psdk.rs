use super::{Incoming, TraitIncoming};
use crate::models::{
    configuration::{psdk::PsdkConfig, Config},
    outgoing::{
        sync_psdk::SyncPsdkOutgoing, sync_start_state::SyncStartStateOutgoing, Outgoing,
        OutgoingState, OutgoingType,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct SyncPsdkIncoming {}

impl SyncPsdkIncoming {
    pub fn new() -> Incoming {
        Incoming::SyncPsdk(Self {})
    }
}

impl TraitIncoming for SyncPsdkIncoming {
    fn name() -> String {
        "SyncPsdk".into()
    }

    async fn run(&self, send_type: OutgoingType) -> Outgoing {
        // Send state
        SyncStartStateOutgoing::new_psdk().send(&send_type);
        // Run sync
        if Config::save_psdk(PsdkConfig::search().await) {
            SyncPsdkOutgoing::new(OutgoingState::Success)
        } else {
            SyncPsdkOutgoing::new(OutgoingState::Info)
        }
    }
}
