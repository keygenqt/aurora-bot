use serde::{Deserialize, Serialize};

use crate::models::outgoing::{app_info::OutgoingAppInfo, Outgoing, OutgoingType};

use super::{Incoming, TraitIncoming};

// Incoming: step 1
#[derive(Serialize, Deserialize, Clone)]
pub struct IncomingAppInfo {}

// Incoming: step 2
impl IncomingAppInfo {
    pub fn new() -> Incoming {
        Incoming::AppInfo(Self {})
    }
}

// Incoming: step 3
impl TraitIncoming for IncomingAppInfo {
    fn name() -> String {
        "AppInfo".into()
    }

    async fn run(&self, _: OutgoingType) -> Result<Outgoing, Box<dyn std::error::Error>> {
        Ok(OutgoingAppInfo::new())
    }
}
