use serde::{Deserialize, Serialize};

use crate::models::outgoing::{app_info::AppInfoOutgoing, Outgoing, OutgoingType};

use super::{Incoming, TraitIncoming};

// Incoming: step 1
#[derive(Serialize, Deserialize, Clone)]
pub struct AppInfoIncoming {}

// Incoming: step 2
impl AppInfoIncoming {
    pub fn new() -> Incoming {
        Incoming::AppInfo(Self {})
    }
}

// Incoming: step 3
impl TraitIncoming for AppInfoIncoming {
    fn name() -> String {
        "AppInfo".into()
    }

    async fn run(&self, _: OutgoingType) -> Outgoing {
        AppInfoOutgoing::new()
    }
}
