use serde::{Deserialize, Serialize};

use crate::service::{dbus::server::ServerDbus, websocket::client::ClientWebsocket};

/// Send data type
#[derive(Clone)]
pub enum OutgoingType {
    Cli,
    Dbus,
    Websocket,
}

/// Base trait for outgoing
pub trait TraitOutgoing {
    /// Cli print data
    fn print(&self);

    /// Serialize data
    fn to_string(&self) -> String;

    /// Send by type interface
    fn send(&self, send_type: &OutgoingType) {
        match send_type {
            OutgoingType::Cli => self.print(),
            OutgoingType::Dbus => ServerDbus::send(self.to_string()),
            OutgoingType::Websocket => ClientWebsocket::send(self.to_string()),
        }
    }
}

/// Data outgoing format
#[derive(Serialize, Deserialize, Clone)]
pub struct DataOutgoing<T: TraitOutgoing + Serialize> {
    key: String,
    value: T,
}

impl<T: TraitOutgoing + Serialize> DataOutgoing<T> {
    pub fn serialize(name: String, value: T) -> String {
        let data = DataOutgoing { key: name, value };
        serde_json::to_string(&data).expect("Error convert")
    }
}
