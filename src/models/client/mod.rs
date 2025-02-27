use serde::Deserialize;
use serde::Serialize;

/// Common import
pub mod incoming;
pub mod outgoing;

/// Methods import
pub mod app_info {
    pub mod incoming;
    pub mod outgoing;
}
pub mod emulator_close {
    pub mod incoming;
}
pub mod emulator_info {
    pub mod incoming;
    pub mod outgoing;
}
pub mod emulator_open {
    pub mod incoming;
}
pub mod emulator_record_start {
    pub mod incoming;
}
pub mod emulator_record_stop {
    pub mod incoming;
    pub mod outgoing;
}
pub mod emulator_screenshot {
    pub mod incoming;
    pub mod outgoing;
}
pub mod emulator_sync {
    pub mod incoming;
}
pub mod emulator_terminal {
    pub mod incoming;
}
pub mod flutter_available {
    pub mod incoming;
    pub mod outgoing;
}
pub mod flutter_info {
    pub mod incoming;
    pub mod outgoing;
}
pub mod flutter_sync {
    pub mod incoming;
}
pub mod flutter_terminal {
    pub mod incoming;
}
pub mod psdk_available {
    pub mod incoming;
    pub mod outgoing;
}
pub mod psdk_info {
    pub mod incoming;
    pub mod outgoing;
}
pub mod psdk_sync {
    pub mod incoming;
}
pub mod psdk_terminal {
    pub mod incoming;
}
pub mod sdk_available {
    pub mod incoming;
    pub mod outgoing;
}
pub mod sdk_info {
    pub mod incoming;
    pub mod outgoing;
}
pub mod sdk_sync {
    pub mod incoming;
}
pub mod sdk_tools {
    pub mod incoming;
}
pub mod selector {
    pub mod incoming;
    pub mod outgoing {
        pub mod incoming;
        pub mod outgoing;
    }
}
pub mod state_message {
    pub mod incoming;
    pub mod outgoing;
}
pub mod ws_ping {
    pub mod incoming;
    pub mod outgoing;
}

/// Common state client
#[derive(Deserialize, Serialize, Clone)]
pub enum ClientMethodsState {
    Error,
    Info,
    State,
    Success,
    Warning,
    // @todo
    Progress,
}

/// Common keys client
#[derive(Serialize, Deserialize, Clone)]
pub enum ClientMethodsKey {
    AppInfo,
    EmulatorClose,
    EmulatorInfo,
    EmulatorOpen,
    EmulatorRecordStart,
    EmulatorRecordStop,
    EmulatorScreenshot,
    EmulatorSync,
    EmulatorTerminal,
    FlutterAvailable,
    FlutterInfo,
    FlutterSync,
    FlutterTerminal,
    PsdkAvailable,
    PsdkInfo,
    PsdkSync,
    PsdkTerminal,
    SdkAvailable,
    SdkInfo,
    SdkSync,
    SdkTools,
    StateMessage,
    WsPing,
}
