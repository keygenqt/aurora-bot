use colored::Colorize;
use serde::{Deserialize, Serialize};

use super::{Outgoing, TraitOutgoing};

#[derive(Serialize, Deserialize, Clone)]
pub struct WsConnectionOutgoing {
    pub message: String,
}

impl WsConnectionOutgoing {
    pub fn new(message: String) -> Outgoing {
        Outgoing::WsConnection(Self { message })
    }

    pub fn new_ping() -> Outgoing {
        Outgoing::WsConnection(Self {
            message: "ping".into(),
        })
    }
}

impl TraitOutgoing for WsConnectionOutgoing {
    fn print(&self) {
        println!("{}", self.message.green().bold())
    }
}
