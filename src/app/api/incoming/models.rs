use serde::{Deserialize, Serialize};

use crate::app::api::enums::ClientKey;

#[allow(dead_code)]
pub enum Incoming {
    Connection(ConnectionIncoming),
    AppInfo(AppInfoIncoming),
    EmulatorStart(EmulatorStartIncoming),
}

#[derive(Serialize, Deserialize)]
pub struct ConnectionIncoming {
    pub key: ClientKey,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct AppInfoIncoming {
    pub key: ClientKey,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct EmulatorStartIncoming {
    pub key: ClientKey,
    pub message: String,
}
