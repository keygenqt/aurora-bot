use serde::{Deserialize, Serialize};

use crate::models::outgoing::{Outgoing, OutgoingType};
use crate::models::outgoing::error::OutgoingError;
use super::{Incoming, TraitIncoming};

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
        OutgoingError::new("что-то пошло не так".into())
    }
}
