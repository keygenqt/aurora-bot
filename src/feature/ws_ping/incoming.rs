use serde::Deserialize;
use serde::Serialize;

use crate::feature::ClientMethodsKey;
use crate::feature::incoming::TraitIncoming;
use crate::feature::outgoing::OutgoingType;
use crate::feature::outgoing::TraitOutgoing;

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
