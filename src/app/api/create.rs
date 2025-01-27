use super::{enums::{ClientKey, ClientState}, outgoing::models::{AppInfoOutgoing, ConnectionOutgoing, EmulatorStartOutgoing, EmulatorStartStateOutgoing, Outgoing}};

impl Outgoing {
    pub fn connection() -> Outgoing {
        Outgoing::Connection(ConnectionOutgoing {
            key: ClientKey::Connection,
            state: ClientState::Success,
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
            code
        })
    }
}
