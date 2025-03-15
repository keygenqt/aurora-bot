use serde::Deserialize;

use crate::tools::constants;
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
        if constants::DEBUG_JSON {
            print_debug!("{}", value)
        }
        let value = utils::clear_to_model_body(value)?;
        match self {
            ClientMethodsKey::AppAuthLogin => {
                let model = serde_json::from_str::<AppAuthLoginIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::AppAuthLogout => {
                let model = serde_json::from_str::<AppAuthLogoutIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::AppInfo => {
                let model = serde_json::from_str::<AppInfoIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::EmulatorClose => {
                let model = serde_json::from_str::<EmulatorCloseIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::EmulatorInfo => {
                let model = serde_json::from_str::<EmulatorInfoIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::EmulatorOpen => {
                let model = serde_json::from_str::<EmulatorOpenIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::EmulatorPackageInstall => {
                let model = serde_json::from_str::<EmulatorPackageInstallIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::EmulatorPackageRun => {
                let model = serde_json::from_str::<EmulatorPackageRunIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::EmulatorPackageUninstall => {
                let model = serde_json::from_str::<EmulatorPackageUninstallIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::EmulatorRecordStart => {
                let model = serde_json::from_str::<EmulatorRecordStartIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::EmulatorRecordStop => {
                let model = serde_json::from_str::<EmulatorRecordStopIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::EmulatorScreenshot => {
                let model = serde_json::from_str::<EmulatorScreenshotIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::EmulatorSync => {
                let model = serde_json::from_str::<EmulatorSyncIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::EmulatorTerminal => {
                let model = serde_json::from_str::<EmulatorTerminalIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::EmulatorUpload => {
                let model = serde_json::from_str::<EmulatorUploadIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::FlutterAvailable => {
                let model = serde_json::from_str::<FlutterAvailableIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::FlutterDownload => {
                let model = serde_json::from_str::<FlutterDownloadIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::FlutterInfo => {
                let model = serde_json::from_str::<FlutterInfoIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::FlutterSync => {
                let model = serde_json::from_str::<FlutterSyncIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::FlutterTerminal => {
                let model = serde_json::from_str::<FlutterTerminalIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::PsdkAvailable => {
                let model = serde_json::from_str::<PsdkAvailableIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::PsdkDownload => {
                let model = serde_json::from_str::<PsdkDownloadIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::PsdkInfo => {
                let model = serde_json::from_str::<PsdkInfoIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::PsdkSync => {
                let model = serde_json::from_str::<PsdkSyncIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::PsdkTerminal => {
                let model = serde_json::from_str::<PsdkTerminalIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::SdkAvailable => {
                let model = serde_json::from_str::<SdkAvailableIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::SdkDownload => {
                let model = serde_json::from_str::<SdkDownloadIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::SdkInfo => {
                let model = serde_json::from_str::<SdkInfoIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::SdkSync => {
                let model = serde_json::from_str::<SdkSyncIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::SdkTools => {
                let model = serde_json::from_str::<SdkToolsIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::StateMessage => {
                let model = serde_json::from_str::<StateMessageIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::WsPing => {
                let model = serde_json::from_str::<WsPingIncoming>(&value)?;
                Ok(Box::new(model))
            }
        }
    }
}
