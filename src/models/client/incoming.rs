use serde::Deserialize;

use crate::models::client::app_open_dir::incoming::AppOpenDirIncoming;
use crate::tools::macros::print_debug;
use crate::tools::utils;

use super::ClientMethodsKey;
use super::app_auth_login::incoming::AppAuthLoginIncoming;
use super::app_auth_logout::incoming::AppAuthLogoutIncoming;
use super::app_info::incoming::AppInfoIncoming;
use super::emulator_close::incoming::EmulatorCloseIncoming;
use super::emulator_info::incoming::EmulatorInfoIncoming;
use super::emulator_open::incoming::EmulatorOpenIncoming;
use super::emulator_package_install::incoming::EmulatorPackageInstallIncoming;
use super::emulator_package_run::incoming::EmulatorPackageRunIncoming;
use super::emulator_package_uninstall::incoming::EmulatorPackageUninstallIncoming;
use super::emulator_record_start::incoming::EmulatorRecordStartIncoming;
use super::emulator_record_stop::incoming::EmulatorRecordStopIncoming;
use super::emulator_screenshot::incoming::EmulatorScreenshotIncoming;
use super::emulator_sync::incoming::EmulatorSyncIncoming;
use super::emulator_terminal::incoming::EmulatorTerminalIncoming;
use super::emulator_upload::incoming::EmulatorUploadIncoming;
use super::flutter_available::incoming::FlutterAvailableIncoming;
use super::flutter_download::incoming::FlutterDownloadIncoming;
use super::flutter_info::incoming::FlutterInfoIncoming;
use super::flutter_sync::incoming::FlutterSyncIncoming;
use super::flutter_terminal::incoming::FlutterTerminalIncoming;
use super::outgoing::OutgoingType;
use super::outgoing::TraitOutgoing;
use super::psdk_available::incoming::PsdkAvailableIncoming;
use super::psdk_download::incoming::PsdkDownloadIncoming;
use super::psdk_info::incoming::PsdkInfoIncoming;
use super::psdk_sync::incoming::PsdkSyncIncoming;
use super::psdk_terminal::incoming::PsdkTerminalIncoming;
use super::sdk_available::incoming::SdkAvailableIncoming;
use super::sdk_download::incoming::SdkDownloadIncoming;
use super::sdk_info::incoming::SdkInfoIncoming;
use super::sdk_sync::incoming::SdkSyncIncoming;
use super::sdk_tools::incoming::SdkToolsIncoming;
use super::state_message::incoming::StateMessageIncoming;
use super::ws_ping::incoming::WsPingIncoming;

pub trait TraitIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing>;
}

/// Data outgoing format
#[derive(Deserialize)]
pub struct DataIncoming {
    key: ClientMethodsKey,
}

impl DataIncoming {
    pub fn deserialize(value: &String) -> Result<ClientMethodsKey, Box<dyn std::error::Error>> {
        Ok(serde_json::from_str::<DataIncoming>(value)?.key)
    }
}

impl ClientMethodsKey {
    pub fn deserialize(&self, value: &String) -> Result<Box<dyn TraitIncoming>, Box<dyn std::error::Error>> {
        let value = utils::clear_to_model_body(value)?;
        match self {
            ClientMethodsKey::AppAuthLogin => {
                print_debug!("> AppAuthLogin: {}", value);
                let model = serde_json::from_str::<AppAuthLoginIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::AppAuthLogout => {
                print_debug!("> AppAuthLogout: {}", value);
                let model = serde_json::from_str::<AppAuthLogoutIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::AppInfo => {
                print_debug!("> AppInfo: {}", value);
                let model = serde_json::from_str::<AppInfoIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::AppOpenDir => {
                print_debug!("> AppOpenDir: {}", value);
                let model = serde_json::from_str::<AppOpenDirIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::EmulatorClose => {
                print_debug!("> EmulatorClose: {}", value);
                let model = serde_json::from_str::<EmulatorCloseIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::EmulatorInfo => {
                print_debug!("> EmulatorInfo: {}", value);
                let model = serde_json::from_str::<EmulatorInfoIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::EmulatorOpen => {
                print_debug!("> EmulatorOpen: {}", value);
                let model = serde_json::from_str::<EmulatorOpenIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::EmulatorPackageInstall => {
                print_debug!("> EmulatorPackageInstall: {}", value);
                let model = serde_json::from_str::<EmulatorPackageInstallIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::EmulatorPackageRun => {
                print_debug!("> EmulatorPackageRun: {}", value);
                let model = serde_json::from_str::<EmulatorPackageRunIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::EmulatorPackageUninstall => {
                print_debug!("> EmulatorPackageUninstall: {}", value);
                let model = serde_json::from_str::<EmulatorPackageUninstallIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::EmulatorRecordStart => {
                print_debug!("> EmulatorRecordStart: {}", value);
                let model = serde_json::from_str::<EmulatorRecordStartIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::EmulatorRecordStop => {
                print_debug!("> EmulatorRecordStop: {}", value);
                let model = serde_json::from_str::<EmulatorRecordStopIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::EmulatorScreenshot => {
                print_debug!("> EmulatorScreenshot: {}", value);
                let model = serde_json::from_str::<EmulatorScreenshotIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::EmulatorSync => {
                print_debug!("> EmulatorSync: {}", value);
                let model = serde_json::from_str::<EmulatorSyncIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::EmulatorTerminal => {
                print_debug!("> EmulatorTerminal: {}", value);
                let model = serde_json::from_str::<EmulatorTerminalIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::EmulatorUpload => {
                print_debug!("> EmulatorUpload: {}", value);
                let model = serde_json::from_str::<EmulatorUploadIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::FlutterAvailable => {
                print_debug!("> FlutterAvailable: {}", value);
                let model = serde_json::from_str::<FlutterAvailableIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::FlutterDownload => {
                print_debug!("> FlutterDownload: {}", value);
                let model = serde_json::from_str::<FlutterDownloadIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::FlutterInfo => {
                print_debug!("> FlutterInfo: {}", value);
                let model = serde_json::from_str::<FlutterInfoIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::FlutterSync => {
                print_debug!("> FlutterSync: {}", value);
                let model = serde_json::from_str::<FlutterSyncIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::FlutterTerminal => {
                print_debug!("> FlutterTerminal: {}", value);
                let model = serde_json::from_str::<FlutterTerminalIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::PsdkAvailable => {
                print_debug!("> PsdkAvailable: {}", value);
                let model = serde_json::from_str::<PsdkAvailableIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::PsdkDownload => {
                print_debug!("> PsdkDownload: {}", value);
                let model = serde_json::from_str::<PsdkDownloadIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::PsdkInfo => {
                print_debug!("> PsdkInfo: {}", value);
                let model = serde_json::from_str::<PsdkInfoIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::PsdkSync => {
                print_debug!("> PsdkSync: {}", value);
                let model = serde_json::from_str::<PsdkSyncIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::PsdkTerminal => {
                print_debug!("> PsdkTerminal: {}", value);
                let model = serde_json::from_str::<PsdkTerminalIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::SdkAvailable => {
                print_debug!("> SdkAvailable: {}", value);
                let model = serde_json::from_str::<SdkAvailableIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::SdkDownload => {
                print_debug!("> SdkDownload: {}", value);
                let model = serde_json::from_str::<SdkDownloadIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::SdkInfo => {
                print_debug!("> SdkInfo: {}", value);
                let model = serde_json::from_str::<SdkInfoIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::SdkSync => {
                print_debug!("> SdkSync: {}", value);
                let model = serde_json::from_str::<SdkSyncIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::SdkTools => {
                print_debug!("> SdkTools: {}", value);
                let model = serde_json::from_str::<SdkToolsIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::StateMessage => {
                print_debug!("> StateMessage: {}", value);
                let model = serde_json::from_str::<StateMessageIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::WsPing => {
                print_debug!("> WsPing: {}", value);
                let model = serde_json::from_str::<WsPingIncoming>(&value)?;
                Ok(Box::new(model))
            }
        }
    }
}
