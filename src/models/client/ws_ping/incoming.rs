use serde::Deserialize;
use serde::Serialize;

use crate::models::client::incoming::TraitIncoming;
use crate::models::client::outgoing::OutgoingType;
use crate::models::client::outgoing::TraitOutgoing;
use crate::models::client::ClientMethodsKey;

use super::outgoing::WsPingOutgoing;

#[derive(Serialize, Deserialize, Clone)]
pub struct WsPingIncoming {
    message: String,
}

impl WsPingIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::WsPing)
            .unwrap()
            .to_string()
    }
}

impl TraitIncoming for WsPingIncoming {
    fn run(&self, _: OutgoingType) -> Box<dyn TraitOutgoing> {
        WsPingOutgoing::new_message(self.message.clone())
    }
}
