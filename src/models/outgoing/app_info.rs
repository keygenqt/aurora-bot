use serde::{Deserialize, Serialize};

use super::{Outgoing, TraitOutgoing};

// Outgoing: step 1
#[derive(Serialize, Deserialize, Clone)]
pub struct OutgoingAppInfo {
    version: String,
}

// Outgoing: step 2
impl OutgoingAppInfo {
    pub fn new() -> Outgoing {
        Outgoing::AppInfo(Self {
            version: "0.0.1".into(),
        })
    }
}

// Outgoing: step 3
impl TraitOutgoing for OutgoingAppInfo {
    fn print(&self) {
        println!("aurora-bot {}", self.version)
    }
}
