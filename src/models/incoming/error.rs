use serde::{Deserialize, Serialize};

use super::{Incoming, TraitIncoming};
use crate::models::outgoing::error::ErrorOutgoing;
use crate::models::outgoing::{Outgoing, OutgoingType};

#[derive(Serialize, Deserialize, Clone)]
pub struct ErrorIncoming {
    pub code: u64,
    pub message: String,
}

#[allow(dead_code)]
impl ErrorIncoming {
    pub fn new(code: u64, message: String) -> Incoming {
        Incoming::Error(Self { code, message })
    }
}

impl TraitIncoming for ErrorIncoming {
    fn name() -> String {
        "Error".into()
    }

    async fn run(&self, _: OutgoingType) -> Outgoing {
        let message = self.message.clone();
        match self.code {
            404 => ErrorOutgoing::new_info(message),
            _ => ErrorOutgoing::new_error(message),
        }
    }
}
