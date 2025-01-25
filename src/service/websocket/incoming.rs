use serde::{Deserialize, Serialize};

use super::enums::MessageKey;
use super::enums::MessageState;

#[derive(Deserialize, Serialize, Debug)]
pub struct WebsocketIncoming {
    pub key: MessageKey,
    pub message: String,
    pub state: MessageState,
}
