use serde::{Deserialize, Serialize};

use crate::utils::macros::{print_error, print_info, print_warning};

use super::{Outgoing, TraitOutgoing};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum ErrorState {
    Info,
    Error,
    Warning,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OutgoingError {
    pub state: ErrorState,
    pub message: String,
}

impl OutgoingError {
    pub fn new_info(message: String) -> Outgoing {
        Outgoing::Error(Self {
            state: ErrorState::Info,
            message,
        })
    }

    pub fn new_error(message: String) -> Outgoing {
        Outgoing::Error(Self {
            state: ErrorState::Error,
            message,
        })
    }

    #[allow(dead_code)]
    pub fn new_warning(message: String) -> Outgoing {
        Outgoing::Error(Self {
            state: ErrorState::Warning,
            message,
        })
    }
}

impl TraitOutgoing for OutgoingError {
    fn print(&self) {
        let message = &self.message;
        match self.state {
            ErrorState::Info => {
                print_info!(message)
            }
            ErrorState::Error => {
                print_error!(message)
            }
            ErrorState::Warning => {
                print_warning!(message)
            }
        }
    }
}
