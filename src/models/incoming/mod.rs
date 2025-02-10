use super::outgoing::{Outgoing, OutgoingType};
use crate::models::incoming::emulator_close::EmulatorCloseIncoming;
use app_info::AppInfoIncoming;
use dbus_info::DbusInfoIncoming;
use device_info::DeviceInfoIncoming;
use emulator_info::EmulatorInfoIncoming;
use emulator_start::EmulatorStartIncoming;
use error::ErrorIncoming;
use flutter_info::FlutterInfoIncoming;
use psdk_info::PsdkInfoIncoming;
use sdk_info::SdkInfoIncoming;
use serde::{Deserialize, Serialize};
use sync_device::SyncDeviceIncoming;
use sync_emulator::SyncEmulatorIncoming;
use sync_flutter::SyncFlutterIncoming;
use sync_psdk::SyncPsdkIncoming;
use sync_sdk::SyncSdkIncoming;
use ws_connect::WsConnectionIncoming;

pub mod app_info;
pub mod dbus_info;
pub mod device_info;
pub mod emulator_close;
pub mod emulator_info;
pub mod emulator_start;
pub mod error;
pub mod flutter_info;
pub mod psdk_info;
pub mod sdk_info;
pub mod sync_device;
pub mod sync_emulator;
pub mod sync_flutter;
pub mod sync_psdk;
pub mod sync_sdk;
pub mod ws_connect;

pub trait TraitIncoming: Clone {
    fn name() -> String;
    async fn run(&self, send_type: OutgoingType) -> Outgoing;
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Incoming {
    // Incoming: step 4
    AppInfo(AppInfoIncoming),
    DbusInfo(DbusInfoIncoming),
    DeviceInfo(DeviceInfoIncoming),
    EmulatorClose(EmulatorCloseIncoming),
    EmulatorInfo(EmulatorInfoIncoming),
    EmulatorStart(EmulatorStartIncoming),
    Error(ErrorIncoming),
    FlutterInfo(FlutterInfoIncoming),
    PsdkInfo(PsdkInfoIncoming),
    SdkInfo(SdkInfoIncoming),
    SyncDevice(SyncDeviceIncoming),
    SyncEmulator(SyncEmulatorIncoming),
    SyncFlutter(SyncFlutterIncoming),
    SyncPsdk(SyncPsdkIncoming),
    SyncSdk(SyncSdkIncoming),
    WsConnection(WsConnectionIncoming),
}

impl Incoming {
    pub fn name(&self) -> String {
        match self {
            // Incoming: step 5
            Incoming::AppInfo(_) => AppInfoIncoming::name(),
            Incoming::DbusInfo(_) => DbusInfoIncoming::name(),
            Incoming::DeviceInfo(_) => DeviceInfoIncoming::name(),
            Incoming::EmulatorClose(_) => EmulatorCloseIncoming::name(),
            Incoming::EmulatorInfo(_) => EmulatorInfoIncoming::name(),
            Incoming::EmulatorStart(_) => EmulatorStartIncoming::name(),
            Incoming::Error(_) => ErrorIncoming::name(),
            Incoming::FlutterInfo(_) => FlutterInfoIncoming::name(),
            Incoming::PsdkInfo(_) => PsdkInfoIncoming::name(),
            Incoming::SdkInfo(_) => SdkInfoIncoming::name(),
            Incoming::SyncDevice(_) => SyncDeviceIncoming::name(),
            Incoming::SyncEmulator(_) => SyncEmulatorIncoming::name(),
            Incoming::SyncFlutter(_) => SyncFlutterIncoming::name(),
            Incoming::SyncPsdk(_) => SyncPsdkIncoming::name(),
            Incoming::SyncSdk(_) => SyncSdkIncoming::name(),
            Incoming::WsConnection(_) => WsConnectionIncoming::name(),
        }
    }

    pub async fn handler(incoming: Incoming, send_type: OutgoingType) -> Outgoing {
        match incoming {
            // Incoming: step 6
            Incoming::AppInfo(model) => model.run(send_type).await,
            Incoming::DbusInfo(model) => model.run(send_type).await,
            Incoming::DeviceInfo(model) => model.run(send_type).await,
            Incoming::EmulatorClose(model) => model.run(send_type).await,
            Incoming::EmulatorInfo(model) => model.run(send_type).await,
            Incoming::EmulatorStart(model) => model.run(send_type).await,
            Incoming::Error(model) => model.run(send_type).await,
            Incoming::FlutterInfo(model) => model.run(send_type).await,
            Incoming::PsdkInfo(model) => model.run(send_type).await,
            Incoming::SdkInfo(model) => model.run(send_type).await,
            Incoming::SyncDevice(model) => model.run(send_type).await,
            Incoming::SyncEmulator(model) => model.run(send_type).await,
            Incoming::SyncFlutter(model) => model.run(send_type).await,
            Incoming::SyncPsdk(model) => model.run(send_type).await,
            Incoming::SyncSdk(model) => model.run(send_type).await,
            Incoming::WsConnection(model) => model.run(send_type).await,
        }
    }

    pub fn convert(value: String) -> Result<Incoming, Box<dyn std::error::Error>> {
        match serde_json::from_str::<Incoming>(&value) {
            Ok(incoming) => Ok(incoming),
            Err(error) => Err(error)?,
        }
    }
}
