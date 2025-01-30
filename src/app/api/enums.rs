use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum ClientState {
    Success,
    Error,
    Info,
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub enum CommandKey {
    // Common
    Error,
    AppInfo,
    EmulatorStart,
    EmulatorStartState,
    // Websocket
    Connection,
    // D-Bus
    ApiInfo,
}

impl fmt::Display for CommandKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub enum SendType {
    Cli,
    Dbus,
    Websocket,
}
