use crate::{
    app::api::enums::{ClientState, CommandKey},
    models::outgoing::{
        ApiInfoOutgoing, AppInfoOutgoing, ConnectionOutgoing, EmulatorStartOutgoing,
        EmulatorStartStateOutgoing, ErrorOutgoing,
    },
};

#[allow(dead_code)]
pub enum Outgoing {
    // Common
    Error(ErrorOutgoing),
    AppInfo(AppInfoOutgoing),
    EmulatorStart(EmulatorStartOutgoing),
    EmulatorStartState(EmulatorStartStateOutgoing),
    // Websocket
    Connection(ConnectionOutgoing),
    // D-Bus
    ApiInfo(ApiInfoOutgoing),
}

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

    pub fn emulator_start(state: ClientState, message: String) -> Outgoing {
        Outgoing::EmulatorStart(EmulatorStartOutgoing {
            key: CommandKey::EmulatorStart,
            state,
            message,
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
