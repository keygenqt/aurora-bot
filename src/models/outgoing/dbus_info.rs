use serde::{Deserialize, Serialize};

use super::{Outgoing, TraitOutgoing};

#[derive(Serialize, Deserialize, Clone)]
pub struct OutgoingDbusInfo {
    version: String,
}

impl OutgoingDbusInfo {
    pub fn new() -> Outgoing {
        Outgoing::DbusInfo(Self {
            version: "0.0.1".into(),
        })
    }
}

impl TraitOutgoing for OutgoingDbusInfo {
    fn print(&self) {
        println!("api-dbus {}", self.version)
    }
}
