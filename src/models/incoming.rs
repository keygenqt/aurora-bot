use serde::{Deserialize, Serialize};

use crate::app::api::enums::CommandKey;

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
