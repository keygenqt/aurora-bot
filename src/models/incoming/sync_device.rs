use super::{Incoming, TraitIncoming};
use crate::models::{
    configuration::{device::DeviceConfig, Config},
    outgoing::{
        sync_device::SyncDeviceOutgoing, sync_start_state::SyncStartStateOutgoing, Outgoing,
        OutgoingState, OutgoingType,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct SyncDeviceIncoming {}

impl SyncDeviceIncoming {
    pub fn new() -> Incoming {
        Incoming::SyncDevice(Self {})
    }
}

impl TraitIncoming for SyncDeviceIncoming {
    fn name() -> String {
        "SyncDevice".into()
    }

    async fn run(&self, send_type: OutgoingType) -> Outgoing {
        // Send state
        SyncStartStateOutgoing::new_device().send(&send_type);
        // Run sync
        if Config::save_device(DeviceConfig::search().await) {
            SyncDeviceOutgoing::new(OutgoingState::Success)
        } else {
            SyncDeviceOutgoing::new(OutgoingState::Info)
        }
    }
}
