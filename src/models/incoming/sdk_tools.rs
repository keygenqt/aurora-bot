use crate::models::outgoing::{sdk_tools::SdkToolsOutgoing, Outgoing, OutgoingState, OutgoingType};

use super::{Incoming, TraitIncoming};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct SdkToolsIncoming {}

impl SdkToolsIncoming {
    pub fn new() -> Incoming {
        Incoming::SdkTools(Self {})
    }
}

impl TraitIncoming for SdkToolsIncoming {
    fn name() -> String {
        "SdkTools".into()
    }

    async fn run(&self, _: OutgoingType) -> Outgoing {
        // @todo
        SdkToolsOutgoing::new(OutgoingState::Error)
    }
}
