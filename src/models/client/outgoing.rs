use serde::Deserialize;
use serde::Serialize;

use crate::service::dbus::server::ServerDbus;
use crate::service::websocket::client::ClientWebsocket;
use crate::tools::constants;
use crate::tools::macros::print_debug;

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
    fn to_json(&self) -> String;

    /// Send by type interface
    fn send(&self, send_type: &OutgoingType) {
        match send_type {
            OutgoingType::Cli => self.print(),
            OutgoingType::Dbus => ServerDbus::send(self.to_json()),
            OutgoingType::Websocket => ClientWebsocket::send(self.to_json()),
        }
    }
}

/// Data outgoing format
#[derive(Serialize, Deserialize, Clone)]
pub struct DataOutgoing<T: TraitOutgoing + Serialize> {
    key: String,
    #[serde(alias = "jsonData")]
    #[serde(rename = "jsonData")]
    json_data: T,
}

impl<T: TraitOutgoing + Serialize> DataOutgoing<T> {
    pub fn serialize(name: String, json_data: T) -> String {
        let data = DataOutgoing { key: name, json_data };
        let outgoing = serde_json::to_string(&data).expect("Error convert");
        if constants::DEBUG_JSON {
            print_debug!("{}", serde_json::to_string_pretty(&data).expect("Error convert"))
        }
        outgoing
    }
}
