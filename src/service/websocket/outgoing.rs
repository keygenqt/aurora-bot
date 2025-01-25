use erased_serde::serialize_trait_object;
use reqwest_websocket::Message;
use serde::{Deserialize, Serialize};

use super::enums::MessageKey;
use super::enums::MessageState;

///////////////////////////////////////////
/// Answers after execute
#[derive(Deserialize, Serialize, Debug)]
pub struct EmulatorStartMessage {
    pub state: MessageState,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AppInfoMessage {
    pub state: MessageState,
    pub version: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EmptyMessage {}

///////////////////////////////////////////
/// Common object message
#[derive(Serialize)]
pub struct WebsocketMessage {
    pub key: MessageKey,
    pub value: Box<dyn MessageTrait>,
}

impl WebsocketMessage {
    pub fn connection() -> WebsocketMessage {
        WebsocketMessage {
            key: MessageKey::Connection,
            value: Box::new(EmptyMessage {}),
        }
    }

    pub fn emulator_start(state: MessageState, message: &str) -> WebsocketMessage {
        WebsocketMessage {
            key: MessageKey::EmulatorStart,
            value: Box::new(EmulatorStartMessage {
                state,
                message: message.into(),
            }),
        }
    }

    pub fn app_info() -> WebsocketMessage {
        WebsocketMessage {
            key: MessageKey::AppInfo,
            value: Box::new(AppInfoMessage {
                state: MessageState::Success,
                version: "0.0.1".into(),
            }),
        }
    }

    pub fn empty() -> WebsocketMessage {
        WebsocketMessage {
            key: MessageKey::Empty,
            value: Box::new(EmptyMessage {}),
        }
    }

    pub fn to_message(&self) -> Message {
        Message::Text(serde_json::to_string(&self).unwrap())
    }
}

///////////////////////////////////////////
/// Traits
pub trait MessageTrait: erased_serde::Serialize {}
// Any variant answer
impl MessageTrait for AppInfoMessage {}
impl MessageTrait for EmulatorStartMessage {}
impl MessageTrait for EmptyMessage {}
// Init
serialize_trait_object!(MessageTrait);
