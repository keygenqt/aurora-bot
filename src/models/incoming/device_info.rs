use crate::models::{
    device::model::DeviceModel,
    outgoing::{device_info::DeviceInfoOutgoing, Outgoing, OutgoingType},
};

use super::{Incoming, TraitIncoming};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct DeviceInfoIncoming {}

impl DeviceInfoIncoming {
    pub fn new() -> Incoming {
        Incoming::DeviceInfo(Self {})
    }
}

impl TraitIncoming for DeviceInfoIncoming {
    fn name() -> String {
        "DeviceInfo".into()
    }

    async fn run(&self, _: OutgoingType) -> Outgoing {
        DeviceInfoOutgoing::new(DeviceModel::search().await)
    }
}
