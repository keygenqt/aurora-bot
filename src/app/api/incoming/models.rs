use serde::{Deserialize, Serialize};

use crate::app::api::enums::CommandKey;

#[allow(dead_code)]
#[derive(Clone)]
pub enum Incoming {
    // Common
    AppInfo(AppInfoIncoming),
    EmulatorStart(EmulatorStartIncoming),
    // Websocket
    Connection(ConnectionIncoming),
    // D-Bus
    ApiInfo(ApiInfoIncoming),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AppInfoIncoming {
    pub key: CommandKey,
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EmulatorStartIncoming {
    pub key: CommandKey,
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConnectionIncoming {
    pub key: CommandKey,
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ApiInfoIncoming {
    pub key: CommandKey,
    pub message: String,
}
