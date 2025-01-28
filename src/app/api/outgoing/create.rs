use crate::app::api::enums::{ClientKey, ClientState};

use super::models::{
    AppInfoOutgoing, ConnectionOutgoing, EmulatorStartOutgoing, EmulatorStartStateOutgoing,
    ErrorOutgoing, Outgoing,
};

impl Outgoing {
    #[allow(dead_code)]
    pub fn error(message: String) -> Outgoing {
        Outgoing::Error(ErrorOutgoing { message })
    }

    pub fn connection(message: String) -> Outgoing {
        Outgoing::Connection(ConnectionOutgoing {
            key: ClientKey::Connection,
            state: ClientState::Success,
            message,
        })
    }

    pub fn app_info() -> Outgoing {
        Outgoing::AppInfo(AppInfoOutgoing {
            key: ClientKey::AppInfo,
            state: ClientState::Success,
            version: String::from("0.0.1"),
        })
    }

    pub fn emulator_start() -> Outgoing {
        Outgoing::EmulatorStart(EmulatorStartOutgoing {
            key: ClientKey::EmulatorStart,
            state: ClientState::Success,
        })
    }

    pub fn emulator_start_state(code: u64) -> Outgoing {
        Outgoing::EmulatorStartState(EmulatorStartStateOutgoing {
            key: ClientKey::EmulatorStartState,
            state: ClientState::Info,
            code,
        })
    }
}
