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
use crate::tools::macros::tr;

use super::incoming::StateMessageIncoming;

#[derive(Serialize, Deserialize, Clone)]
pub struct StateMessageOutgoing {
    state: ClientMethodsState,
    message: String,
}

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

    pub fn new_progress(message: String) -> Box<StateMessageOutgoing> {
        Box::new(Self {
            state: ClientMethodsState::Progress,
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
            ClientMethodsState::Progress => {
                if message != "0" {
                    if let Some(mut t) = term::stdout() {
                        let _ = t.cursor_up();
                        let _ = t.delete_line();
                    }
                }
                let message = tr!("прогресс: {}%", message);
                print_state!(message);
            }
        }
    }

    fn to_json(&self) -> String {
        DataOutgoing::serialize(StateMessageIncoming::name(), self.clone())
    }
}
