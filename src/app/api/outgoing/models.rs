use serde::{Deserialize, Serialize};

use crate::app::api::enums::{ClientKey, ClientState};

#[allow(dead_code)]
pub enum Outgoing {
    Connection(ConnectionOutgoing),
    AppInfo(AppInfoOutgoing),
    EmulatorStart(EmulatorStartOutgoing),
    EmulatorStartState(EmulatorStartStateOutgoing),
}

#[derive(Serialize, Deserialize)]
pub struct ConnectionOutgoing {
    pub key: ClientKey,
    pub state: ClientState,
}

#[derive(Serialize, Deserialize)]
pub struct AppInfoOutgoing {
    pub key: ClientKey,
    pub state: ClientState,
    pub version: String,
}

#[derive(Serialize, Deserialize)]
pub struct EmulatorStartOutgoing {
    pub key: ClientKey,
    pub state: ClientState,
}

#[derive(Serialize, Deserialize)]
pub struct EmulatorStartStateOutgoing {
    pub key: ClientKey,
    pub state: ClientState,
    pub code: u64,
}
