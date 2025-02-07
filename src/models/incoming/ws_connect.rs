use serde::{Deserialize, Serialize};

use crate::models::outgoing::{ws_connect::WsConnectionOutgoing, Outgoing, OutgoingType};

use super::{Incoming, TraitIncoming};

#[derive(Serialize, Deserialize, Clone)]
pub struct WsConnectionIncoming {
    pub message: String,
}

#[allow(dead_code)]
impl WsConnectionIncoming {
    pub fn new() -> Incoming {
        Incoming::WsConnection(Self {
            message: "default".into(),
        })
    }
}

impl TraitIncoming for WsConnectionIncoming {
    fn name() -> String {
        "WsConnection".into()
    }

    async fn run(&self, _: OutgoingType) -> Outgoing {
        WsConnectionOutgoing::new(self.message.clone())
    }
}
