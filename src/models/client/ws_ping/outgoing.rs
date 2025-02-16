use serde::Deserialize;
use serde::Serialize;

use crate::models::client::outgoing::DataOutgoing;
use crate::models::client::outgoing::TraitOutgoing;
use crate::tools::macros::print_error;
use crate::tools::macros::print_success;

use super::incoming::WsPingIncoming;

#[derive(Serialize, Deserialize, Clone)]
pub struct WsPingOutgoing {
    message: Option<String>,
}

impl WsPingOutgoing {
    pub fn new() -> Box<WsPingOutgoing> {
        Box::new(Self { message: None })
    }

    pub fn new_message(message: String) -> Box<WsPingOutgoing> {
        Box::new(Self { message: Some(message) })
    }
}

impl TraitOutgoing for WsPingOutgoing {
    fn print(&self) {
        if let Some(message) = &self.message {
            print_success!(message)
        } else {
            print_error!("ошибка при получении данных")
        }
    }

    fn to_string(&self) -> String {
        DataOutgoing::serialize(WsPingIncoming::name(), self.clone())
    }
}
