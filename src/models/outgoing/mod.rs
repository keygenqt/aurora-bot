use crate::models::outgoing::emulator_close::EmulatorCloseOutgoing;
use crate::models::outgoing::emulator_close_state::EmulatorCloseStateOutgoing;
use crate::service::{dbus::server::ServerDbus, websocket::client::ClientWebsocket};
use app_info::AppInfoOutgoing;
use dbus_info::DbusInfoOutgoing;
use device_info::DeviceInfoOutgoing;
use device_terminal::DeviceTerminalOutgoing;
use emulator_info::EmulatorInfoOutgoing;
use emulator_start::*;
use emulator_start_state::EmulatorStartStateOutgoing;
use emulator_terminal::EmulatorTerminalOutgoing;
use error::ErrorOutgoing;
use flutter_info::FlutterInfoOutgoing;
use flutter_terminal::FlutterTerminalOutgoing;
use psdk_info::PsdkInfoOutgoing;
use psdk_terminal::PsdkTerminalOutgoing;
use sdk_info::SdkInfoOutgoing;
use sdk_tools::SdkToolsOutgoing;
use serde::{Deserialize, Serialize};
use sync_device::SyncDeviceOutgoing;
use sync_emulator::SyncEmulatorOutgoing;
use sync_flutter::SyncFlutterOutgoing;
use sync_psdk::SyncPsdkOutgoing;
use sync_sdk::SyncSdkOutgoing;
use sync_start_state::SyncStartStateOutgoing;
use ws_connect::WsConnectionOutgoing;

pub mod app_info;
pub mod dbus_info;
pub mod device_info;
pub mod device_terminal;
pub mod emulator_close;
pub mod emulator_close_state;
pub mod emulator_info;
pub mod emulator_start;
pub mod emulator_start_state;
pub mod emulator_terminal;
pub mod error;
pub mod flutter_info;
pub mod flutter_terminal;
pub mod psdk_info;
pub mod psdk_terminal;
pub mod sdk_info;
pub mod sdk_tools;
pub mod sync_device;
pub mod sync_emulator;
pub mod sync_flutter;
pub mod sync_psdk;
pub mod sync_sdk;
pub mod sync_start_state;
pub mod ws_connect;

#[derive(Deserialize, Serialize, Clone)]
pub enum OutgoingState {
    Success,
    Error,
    Info,
}

pub enum OutgoingType {
    Cli,
    Dbus,
    Websocket,
}

pub trait TraitOutgoing: Clone {
    fn print(&self);
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Outgoing {
    // Outgoing: step 4
    AppInfo(AppInfoOutgoing),
    DbusInfo(DbusInfoOutgoing),
    DeviceInfo(DeviceInfoOutgoing),
    DeviceTerminal(DeviceTerminalOutgoing),
    EmulatorClose(EmulatorCloseOutgoing),
    EmulatorInfo(EmulatorInfoOutgoing),
    EmulatorCloseState(EmulatorCloseStateOutgoing),
    EmulatorStart(EmulatorStartOutgoing),
    EmulatorStartState(EmulatorStartStateOutgoing),
    EmulatorTerminal(EmulatorTerminalOutgoing),
    Error(ErrorOutgoing),
    FlutterInfo(FlutterInfoOutgoing),
    FlutterTerminal(FlutterTerminalOutgoing),
    PsdkInfo(PsdkInfoOutgoing),
    PsdkTerminal(PsdkTerminalOutgoing),
    SdkInfo(SdkInfoOutgoing),
    SdkTools(SdkToolsOutgoing),
    SyncDevice(SyncDeviceOutgoing),
    SyncEmulator(SyncEmulatorOutgoing),
    SyncFlutter(SyncFlutterOutgoing),
    SyncPsdk(SyncPsdkOutgoing),
    SyncSdk(SyncSdkOutgoing),
    SyncStartState(SyncStartStateOutgoing),
    WsConnection(WsConnectionOutgoing),
}

impl Outgoing {
    pub fn print(&self) {
        match self {
            // Outgoing: step 5
            Outgoing::AppInfo(model) => model.print(),
            Outgoing::DbusInfo(model) => model.print(),
            Outgoing::DeviceInfo(model) => model.print(),
            Outgoing::DeviceTerminal(model) => model.print(),
            Outgoing::EmulatorCloseState(model) => model.print(),
            Outgoing::EmulatorClose(model) => model.print(),
            Outgoing::EmulatorInfo(model) => model.print(),
            Outgoing::EmulatorStartState(model) => model.print(),
            Outgoing::EmulatorStart(model) => model.print(),
            Outgoing::EmulatorTerminal(model) => model.print(),
            Outgoing::Error(model) => model.print(),
            Outgoing::FlutterInfo(model) => model.print(),
            Outgoing::FlutterTerminal(model) => model.print(),
            Outgoing::PsdkInfo(model) => model.print(),
            Outgoing::PsdkTerminal(model) => model.print(),
            Outgoing::SdkInfo(model) => model.print(),
            Outgoing::SdkTools(model) => model.print(),
            Outgoing::SyncDevice(model) => model.print(),
            Outgoing::SyncEmulator(model) => model.print(),
            Outgoing::SyncFlutter(model) => model.print(),
            Outgoing::SyncPsdk(model) => model.print(),
            Outgoing::SyncSdk(model) => model.print(),
            Outgoing::SyncStartState(model) => model.print(),
            Outgoing::WsConnection(model) => model.print(),
        }
    }

    pub fn send(&self, send_type: &OutgoingType) {
        match send_type {
            OutgoingType::Cli => self.print(),
            OutgoingType::Dbus => ServerDbus::send(self),
            OutgoingType::Websocket => ClientWebsocket::send(self),
        }
    }

    pub fn to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(serde_json::to_string(&self)?)
    }
}
