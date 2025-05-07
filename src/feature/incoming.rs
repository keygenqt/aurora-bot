use serde::Deserialize;

use crate::feature::app_open_dir::incoming::AppOpenDirIncoming;
use crate::feature::app_open_file::incoming::AppOpenFileIncoming;
use crate::feature::demo_app_info::incoming::DemoAppInfoIncoming;
use crate::feature::device_info::incoming::DeviceInfoIncoming;
use crate::feature::device_package_install::incoming::DevicePackageInstallIncoming;
use crate::feature::device_package_run::incoming::DevicePackageRunIncoming;
use crate::feature::device_package_uninstall::incoming::DevicePackageUninstallIncoming;
use crate::feature::device_screenshot::incoming::DeviceScreenshotIncoming;
use crate::feature::device_sync::incoming::DeviceSyncIncoming;
use crate::feature::device_terminal::incoming::DeviceTerminalIncoming;
use crate::feature::device_upload::incoming::DeviceUploadIncoming;
use crate::feature::flutter_install::incoming::FlutterInstallIncoming;
use crate::feature::flutter_project_format::incoming::FlutterProjectFormatIncoming;
use crate::feature::flutter_project_report::incoming::FlutterProjectReportIncoming;
use crate::feature::flutter_uninstall::incoming::FlutterUninstallIncoming;
use crate::feature::psdk_install::incoming::PsdkInstallIncoming;
use crate::feature::psdk_package_sign::incoming::PsdkPackageSignIncoming;
use crate::feature::psdk_target_package_find::incoming::PsdkTargetPackageFindIncoming;
use crate::feature::psdk_target_package_install::incoming::PsdkTargetPackageInstallIncoming;
use crate::feature::psdk_target_package_uninstall::incoming::PsdkTargetPackageUninstallIncoming;
use crate::feature::psdk_uninstall::incoming::PsdkUninstallIncoming;
use crate::feature::sdk_ide_close::incoming::SdkIdeCloseIncoming;
use crate::feature::sdk_ide_open::incoming::SdkIdeOpenIncoming;
use crate::feature::sdk_install::incoming::SdkInstallIncoming;
use crate::feature::sdk_project_format::incoming::SdkProjectFormatIncoming;
use crate::feature::sdk_uninstall::incoming::SdkUninstallIncoming;
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

    pub fn get_model(value: &String) -> Result<String, Box<dyn std::error::Error>> {
        utils::clear_to_model_body(value)
    }
}

impl ClientMethodsKey {
    pub fn deserialize(&self, value: &String) -> Result<Box<dyn TraitIncoming>, Box<dyn std::error::Error>> {
        let value = DataIncoming::get_model(value)?;
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
            ClientMethodsKey::AppOpenFile => {
                print_debug!("> AppOpenFile: {}", value);
                let model = serde_json::from_str::<AppOpenFileIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::DemoAppInfo => {
                print_debug!("> DemoAppInfo: {}", value);
                let model = serde_json::from_str::<DemoAppInfoIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::DeviceInfo => {
                print_debug!("> DeviceInfo: {}", value);
                let model = serde_json::from_str::<DeviceInfoIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::DevicePackageInstall => {
                print_debug!("> DevicePackageInstall: {}", value);
                let model = serde_json::from_str::<DevicePackageInstallIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::DevicePackageRun => {
                print_debug!("> DevicePackageRun: {}", value);
                let model = serde_json::from_str::<DevicePackageRunIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::DevicePackageUninstall => {
                print_debug!("> DevicePackageUninstall: {}", value);
                let model = serde_json::from_str::<DevicePackageUninstallIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::DeviceScreenshot => {
                print_debug!("> DeviceScreenshot: {}", value);
                let model = serde_json::from_str::<DeviceScreenshotIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::DeviceSync => {
                print_debug!("> DeviceSync: {}", value);
                let model = serde_json::from_str::<DeviceSyncIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::DeviceTerminal => {
                print_debug!("> DeviceTerminal: {}", value);
                let model = serde_json::from_str::<DeviceTerminalIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::DeviceUpload => {
                print_debug!("> DeviceUpload: {}", value);
                let model = serde_json::from_str::<DeviceUploadIncoming>(&value)?;
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
            ClientMethodsKey::FlutterInstall => {
                print_debug!("> FlutterInstall: {}", value);
                let model = serde_json::from_str::<FlutterInstallIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::FlutterProjectFormat => {
                print_debug!("> FlutterProjectFormat: {}", value);
                let model = serde_json::from_str::<FlutterProjectFormatIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::FlutterProjectReport => {
                print_debug!("> FlutterProjectReport: {}", value);
                let model = serde_json::from_str::<FlutterProjectReportIncoming>(&value)?;
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
            ClientMethodsKey::FlutterUninstall => {
                print_debug!("> FlutterUninstall: {}", value);
                let model = serde_json::from_str::<FlutterUninstallIncoming>(&value)?;
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
            ClientMethodsKey::PsdkInstall => {
                print_debug!("> PsdkInstall: {}", value);
                let model = serde_json::from_str::<PsdkInstallIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::PsdkPackageSign => {
                print_debug!("> PsdkPackageSign: {}", value);
                let model = serde_json::from_str::<PsdkPackageSignIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::PsdkSync => {
                print_debug!("> PsdkSync: {}", value);
                let model = serde_json::from_str::<PsdkSyncIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::PsdkTargetPackageInstall => {
                print_debug!("> PsdkTargetPackageInstall: {}", value);
                let model = serde_json::from_str::<PsdkTargetPackageInstallIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::PsdkTargetPackageFind => {
                print_debug!("> PsdkTargetPackageFind: {}", value);
                let model = serde_json::from_str::<PsdkTargetPackageFindIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::PsdkTargetPackageUninstall => {
                print_debug!("> PsdkTargetPackageUninstall: {}", value);
                let model = serde_json::from_str::<PsdkTargetPackageUninstallIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::PsdkTerminal => {
                print_debug!("> PsdkTerminal: {}", value);
                let model = serde_json::from_str::<PsdkTerminalIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::PsdkUninstall => {
                print_debug!("> PsdkUninstall: {}", value);
                let model = serde_json::from_str::<PsdkUninstallIncoming>(&value)?;
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
            ClientMethodsKey::SdkIdeClose => {
                print_debug!("> SdkIdeClose: {}", value);
                let model = serde_json::from_str::<SdkIdeCloseIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::SdkIdeOpen => {
                print_debug!("> SdkIdeOpen: {}", value);
                let model = serde_json::from_str::<SdkIdeOpenIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::SdkInfo => {
                print_debug!("> SdkInfo: {}", value);
                let model = serde_json::from_str::<SdkInfoIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::SdkInstall => {
                print_debug!("> SdkInstall: {}", value);
                let model = serde_json::from_str::<SdkInstallIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::SdkProjectFormat => {
                print_debug!("> SdkProjectFormat: {}", value);
                let model = serde_json::from_str::<SdkProjectFormatIncoming>(&value)?;
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
            ClientMethodsKey::SdkUninstall => {
                print_debug!("> SdkUninstall: {}", value);
                let model = serde_json::from_str::<SdkUninstallIncoming>(&value)?;
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
