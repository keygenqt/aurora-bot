use serde::{Deserialize, Serialize};

use crate::utils::macros::print_info;

use super::{Outgoing, TraitOutgoing};

#[derive(Deserialize, Serialize, Clone)]
pub enum EmulatorStartState {
    Search,
    Start,
    Loading,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EmulatorStartStateOutgoing {
    pub state: EmulatorStartState,
}

impl EmulatorStartStateOutgoing {
    pub fn new_search() -> Outgoing {
        Outgoing::EmulatorStartState(Self {
            state: EmulatorStartState::Search,
        })
    }

    pub fn new_start() -> Outgoing {
        Outgoing::EmulatorStartState(Self {
            state: EmulatorStartState::Start,
        })
    }

    pub fn new_loading() -> Outgoing {
        Outgoing::EmulatorStartState(Self {
            state: EmulatorStartState::Loading,
        })
    }
}

impl TraitOutgoing for EmulatorStartStateOutgoing {
    fn print(&self) {
        match self.state {
            EmulatorStartState::Search => print_info!("поиск эмулятора..."),
            EmulatorStartState::Start => print_info!("запуск эмулятора..."),
            EmulatorStartState::Loading => print_info!("ожидаем подключение..."),
        }
    }
}
