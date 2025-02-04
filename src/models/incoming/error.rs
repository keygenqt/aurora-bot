use serde::{Deserialize, Serialize};

use crate::models::outgoing::{Outgoing, OutgoingType};

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

    async fn run(&self, _: OutgoingType) -> Result<Outgoing, Box<dyn std::error::Error>> {
        Err("")?
    }
}
