use serde::{Deserialize, Serialize};

use crate::utils::macros::print_info;

use super::{Outgoing, TraitOutgoing};

#[derive(Deserialize, Serialize, Clone)]
pub enum EmulatorCloseState {
    Search,
    Close,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EmulatorCloseStateOutgoing {
    pub state: EmulatorCloseState,
}

impl EmulatorCloseStateOutgoing {
    pub fn new_search() -> Outgoing {
        Outgoing::EmulatorCloseState(Self {
            state: EmulatorCloseState::Search,
        })
    }

    pub fn new_close() -> Outgoing {
        Outgoing::EmulatorCloseState(Self {
            state: EmulatorCloseState::Close,
        })
    }
}

impl TraitOutgoing for EmulatorCloseStateOutgoing {
    fn print(&self) {
        match self.state {
            EmulatorCloseState::Search => print_info!("поиск эмулятора..."),
            EmulatorCloseState::Close => print_info!("закрываем эмулятор..."),
        }
    }
}
