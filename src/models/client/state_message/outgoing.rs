use serde::Deserialize;
use serde::Serialize;

use crate::models::client::ClientMethodsState;
use crate::models::client::outgoing::DataOutgoing;
use crate::models::client::outgoing::OutgoingType;
use crate::models::client::outgoing::TraitOutgoing;
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

    pub fn get_state_callback(send_type: &OutgoingType) -> fn(i32) {
        match send_type {
            OutgoingType::Cli => |progress| {
                Self::send_state_common(progress, &OutgoingType::Cli);
            },
            OutgoingType::Dbus => |progress| {
                Self::send_state_common(progress, &OutgoingType::Dbus);
            },
            OutgoingType::Websocket => |progress| {
                Self::send_state_common(progress, &OutgoingType::Websocket);
            },
        }
    }

    fn send_state_common(progress: i32, send_type: &'static OutgoingType) {
        if progress < 0 {
            match progress {
                -1 => StateMessageOutgoing::new_state(tr!("получение данных...")).send(send_type),
                -2 => StateMessageOutgoing::new_state(tr!("причесываем данные...")).send(send_type),
                -3 => StateMessageOutgoing::new_state(tr!("запускаем процесс...")).send(send_type),
                _ => {}
            }
        } else {
            match send_type {
                OutgoingType::Websocket => {
                    if progress % 10 == 0 {
                        StateMessageOutgoing::new_progress(progress.to_string()).send(send_type);
                    }
                }
                _ => StateMessageOutgoing::new_progress(progress.to_string()).send(send_type),
            }
        }
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
