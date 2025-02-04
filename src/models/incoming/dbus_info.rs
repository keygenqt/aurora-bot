use serde::{Deserialize, Serialize};

use crate::models::outgoing::{dbus_info::OutgoingDbusInfo, Outgoing, OutgoingType};

use super::{Incoming, TraitIncoming};

#[derive(Serialize, Deserialize, Clone)]
pub struct IncomingDbusInfo {}

impl IncomingDbusInfo {
    pub fn new() -> Incoming {
        Incoming::DbusInfo(Self {})
    }
}

impl TraitIncoming for IncomingDbusInfo {
    fn name() -> String {
        "DbusInfo".into()
    }

    async fn run(&self, _: OutgoingType) -> Result<Outgoing, Box<dyn std::error::Error>> {
        Ok(OutgoingDbusInfo::new())
    }
}
