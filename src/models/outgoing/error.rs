use serde::{Deserialize, Serialize};

use crate::utils::macros::print_error;

use super::{Outgoing, TraitOutgoing};

#[derive(Serialize, Deserialize, Clone)]
pub struct OutgoingError {
    pub message: String,
}

impl OutgoingError {
    pub fn new(message: String) -> Outgoing {
        Outgoing::Error(Self { message })
    }
}

impl TraitOutgoing for OutgoingError {
    fn print(&self) {
        let message = &self.message;
        print_error!(message)
    }
}
