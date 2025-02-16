use serde::Deserialize;
use serde::Serialize;

use crate::models::client::outgoing::DataOutgoing;
use crate::models::client::outgoing::TraitOutgoing;
use crate::models::client::ClientMethodsState;
use crate::tools::macros::print_error;
use crate::tools::macros::print_info;
use crate::tools::macros::print_state;
use crate::tools::macros::print_success;
use crate::tools::macros::print_warning;

use super::incoming::StateMessageIncoming;

#[derive(Serialize, Deserialize, Clone)]
pub struct StateMessageOutgoing {
    state: ClientMethodsState,
    message: String,
}

#[allow(dead_code)]
impl StateMessageOutgoing {
    pub fn new_error(message: String) -> Box<StateMessageOutgoing> {
        Box::new(Self {
            state: ClientMethodsState::Error,
            message,
        })
    }

    pub fn new_info(message: String) -> Box<StateMessageOutgoing> {
        Box::new(Self {
            state: ClientMethodsState::Info,
            message,
        })
    }

    pub fn new_state(message: String) -> Box<StateMessageOutgoing> {
        Box::new(Self {
            state: ClientMethodsState::State,
            message,
        })
    }

    pub fn new_success(message: String) -> Box<StateMessageOutgoing> {
        Box::new(Self {
            state: ClientMethodsState::Success,
            message,
        })
    }

    pub fn new_warning(message: String) -> Box<StateMessageOutgoing> {
        Box::new(Self {
            state: ClientMethodsState::Warning,
            message,
        })
    }
}

impl TraitOutgoing for StateMessageOutgoing {
    fn print(&self) {
        let message = &self.message;
        match self.state {
            ClientMethodsState::Error => print_error!(message),
            ClientMethodsState::Info => print_info!(message),
            ClientMethodsState::State => print_state!(message),
            ClientMethodsState::Success => print_success!(message),
            ClientMethodsState::Warning => print_warning!(message),
        }
    }

    fn to_string(&self) -> String {
        DataOutgoing::serialize(StateMessageIncoming::name(), self.clone())
    }
}
