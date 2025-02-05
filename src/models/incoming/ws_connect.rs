use serde::{Deserialize, Serialize};

use crate::models::outgoing::{ws_connect::OutgoingWsConnection, Outgoing, OutgoingType};

use super::{Incoming, TraitIncoming};

#[derive(Serialize, Deserialize, Clone)]
pub struct IncomingWsConnection {
    pub message: String,
}

#[allow(dead_code)]
impl IncomingWsConnection {
    pub fn new() -> Incoming {
        Incoming::WsConnection(Self {
            message: "default".into(),
        })
    }
}

impl TraitIncoming for IncomingWsConnection {
    fn name() -> String {
        "WsConnection".into()
    }

    async fn run(&self, _: OutgoingType) -> Outgoing {
        OutgoingWsConnection::new(self.message.clone())
    }
}
