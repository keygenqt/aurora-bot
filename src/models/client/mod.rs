use serde::Deserialize;
use serde::Serialize;

/// Common import
pub mod incoming;
pub mod outgoing;

/// Methods import
pub mod app_auth_login {
    pub mod incoming;
}
pub mod app_auth_logout {
    pub mod incoming;
}
pub mod app_info {
    pub mod incoming;
    pub mod outgoing;
}
pub mod app_open_dir {
    pub mod incoming;
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
pub mod emulator_package_install {
    pub mod incoming;
}
pub mod emulator_package_run {
    pub mod incoming;
}
pub mod emulator_package_uninstall {
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
pub mod emulator_upload {
    pub mod incoming;
}
pub mod flutter_available {
    pub mod incoming;
    pub mod outgoing;
}
pub mod flutter_download {
    pub mod incoming;
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
pub mod psdk_download {
    pub mod incoming;
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
pub mod sdk_download {
    pub mod incoming;
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
    pub mod selects {
        pub mod select_demo_app;
        pub mod select_device;
        pub mod select_emulator;
        pub mod select_emulator_packages;
        pub mod select_flutter_available;
        pub mod select_flutter_installed;
        pub mod select_psdk_available;
        pub mod select_psdk_installed;
        pub mod select_sdk_available;
        pub mod select_sdk_installed;
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
    Progress,
}

/// Common keys client
#[derive(Serialize, Deserialize, Clone)]
pub enum ClientMethodsKey {
    AppAuthLogin,
    AppAuthLogout,
    AppInfo,
    AppOpenDir,
    EmulatorClose,
    EmulatorInfo,
    EmulatorOpen,
    EmulatorPackageInstall,
    EmulatorPackageRun,
    EmulatorPackageUninstall,
    EmulatorRecordStart,
    EmulatorRecordStop,
    EmulatorScreenshot,
    EmulatorSync,
    EmulatorTerminal,
    EmulatorUpload,
    FlutterAvailable,
    FlutterDownload,
    FlutterInfo,
    FlutterSync,
    FlutterTerminal,
    PsdkAvailable,
    PsdkDownload,
    PsdkInfo,
    PsdkSync,
    PsdkTerminal,
    SdkAvailable,
    SdkDownload,
    SdkInfo,
    SdkSync,
    SdkTools,
    StateMessage,
    WsPing,
}
