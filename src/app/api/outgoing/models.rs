use serde::{Deserialize, Serialize};

use crate::app::api::enums::{ClientState, CommandKey};

#[allow(dead_code)]
pub enum Outgoing {
    // Common
    Error(ErrorOutgoing),
    AppInfo(AppInfoOutgoing),
    EmulatorStart(EmulatorStartOutgoing),
    EmulatorStartState(EmulatorStartStateOutgoing),
    // Websocket
    Connection(ConnectionOutgoing),
    // D-Bus
    ApiInfo(ApiInfoOutgoing),
}

#[derive(Serialize, Deserialize)]
pub struct AppInfoOutgoing {
    pub key: CommandKey,
    pub state: ClientState,
    pub version: String,
}

#[derive(Serialize, Deserialize)]
pub struct EmulatorStartOutgoing {
    pub key: CommandKey,
    pub state: ClientState,
}

#[derive(Serialize, Deserialize)]
pub struct EmulatorStartStateOutgoing {
    pub key: CommandKey,
    pub state: ClientState,
    pub code: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ConnectionOutgoing {
    pub key: CommandKey,
    pub state: ClientState,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorOutgoing {
    pub key: CommandKey,
    pub state: ClientState,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiInfoOutgoing {
    pub key: CommandKey,
    pub state: ClientState,
    pub version: String,
}
