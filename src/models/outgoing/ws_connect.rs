use colored::Colorize;
use serde::{Deserialize, Serialize};

use super::{Outgoing, TraitOutgoing};

#[derive(Serialize, Deserialize, Clone)]
pub struct OutgoingWsConnection {
    pub message: String,
}

impl OutgoingWsConnection {
    pub fn new(message: String) -> Outgoing {
        Outgoing::WsConnection(Self { message })
    }

    pub fn new_ping() -> Outgoing {
        Outgoing::WsConnection(Self {
            message: "ping".into(),
        })
    }
}

impl TraitOutgoing for OutgoingWsConnection {
    fn print(&self) {
        println!("{}", self.message.green().bold())
    }
}
