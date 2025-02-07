use serde::{Deserialize, Serialize};

use super::{Outgoing, TraitOutgoing};

// Outgoing: step 1
#[derive(Serialize, Deserialize, Clone)]
pub struct AppInfoOutgoing {
    version: String,
}

// Outgoing: step 2
impl AppInfoOutgoing {
    pub fn new() -> Outgoing {
        Outgoing::AppInfo(Self {
            version: "0.0.1".into(),
        })
    }
}

// Outgoing: step 3
impl TraitOutgoing for AppInfoOutgoing {
    fn print(&self) {
        println!("aurora-bot {}", self.version)
    }
}
