use serde::{Deserialize, Serialize};

use super::{Incoming, TraitIncoming};
use crate::models::outgoing::error::OutgoingError;
use crate::models::outgoing::{Outgoing, OutgoingType};

#[derive(Serialize, Deserialize, Clone)]
pub struct IncomingError {
    pub code: u64,
    pub message: String,
}

#[allow(dead_code)]
impl IncomingError {
    pub fn new(code: u64, message: String) -> Incoming {
        Incoming::Error(Self { code, message })
    }
}

impl TraitIncoming for IncomingError {
    fn name() -> String {
        "Error".into()
    }

    async fn run(&self, _: OutgoingType) -> Outgoing {
        let message = self.message.clone();
        match self.code {
            404 => OutgoingError::new_info(message),
            _ => OutgoingError::new_error(message),
        }
    }
}
