use serde::{Deserialize, Serialize};

use crate::models::outgoing::{dbus_info::DbusInfoOutgoing, Outgoing, OutgoingType};

use super::{Incoming, TraitIncoming};

#[derive(Serialize, Deserialize, Clone)]
pub struct DbusInfoIncoming {}

impl DbusInfoIncoming {
    pub fn new() -> Incoming {
        Incoming::DbusInfo(Self {})
    }
}

impl TraitIncoming for DbusInfoIncoming {
    fn name() -> String {
        "DbusInfo".into()
    }

    async fn run(&self, _: OutgoingType) -> Outgoing {
        DbusInfoOutgoing::new()
    }
}
