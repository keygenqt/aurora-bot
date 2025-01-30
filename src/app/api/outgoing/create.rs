use crate::app::api::enums::{ClientState, CommandKey};

use super::models::{
    ApiInfoOutgoing, AppInfoOutgoing, ConnectionOutgoing, EmulatorStartOutgoing,
    EmulatorStartStateOutgoing, ErrorOutgoing, Outgoing,
};

impl Outgoing {
    pub fn error(message: String) -> Outgoing {
        Outgoing::Error(ErrorOutgoing {
            key: CommandKey::Error,
            state: ClientState::Error,
            message,
        })
    }

    pub fn app_info() -> Outgoing {
        Outgoing::AppInfo(AppInfoOutgoing {
            key: CommandKey::AppInfo,
            state: ClientState::Success,
            version: "0.0.1".into(),
        })
    }

    pub fn emulator_start() -> Outgoing {
        Outgoing::EmulatorStart(EmulatorStartOutgoing {
            key: CommandKey::EmulatorStart,
            state: ClientState::Success,
        })
    }

    pub fn emulator_start_state(code: u64) -> Outgoing {
        Outgoing::EmulatorStartState(EmulatorStartStateOutgoing {
            key: CommandKey::EmulatorStartState,
            state: ClientState::Info,
            code,
        })
    }

    pub fn connection(message: String) -> Outgoing {
        Outgoing::Connection(ConnectionOutgoing {
            key: CommandKey::Connection,
            state: ClientState::Success,
            message,
        })
    }

    pub fn api_info() -> Outgoing {
        Outgoing::ApiInfo(ApiInfoOutgoing {
            key: CommandKey::ApiInfo,
            state: ClientState::Success,
            version: "0.0.1".into(),
        })
    }
}
