use serde::Deserialize;

use crate::tools::utils;

use super::{
    app_info::incoming::AppInfoIncoming, emulator_close::incoming::EmulatorCloseIncoming, emulator_info::incoming::EmulatorInfoIncoming, emulator_open::incoming::EmulatorOpenIncoming, emulator_open_vnc::incoming::EmulatorOpenVncIncoming, emulator_record_disable::incoming::EmulatorRecordDisableIncoming, emulator_record_enable::incoming::EmulatorRecordEnableIncoming, emulator_screenshot::incoming::EmulatorScreenshotIncoming, emulator_sync::incoming::EmulatorSyncIncoming, emulator_terminal::incoming::EmulatorTerminalIncoming, emulator_terminal_root::incoming::EmulatorTerminalRootIncoming, flutter_info::incoming::FlutterInfoIncoming, flutter_sync::incoming::FlutterSyncIncoming, flutter_terminal::incoming::FlutterTerminalIncoming, outgoing::{OutgoingType, TraitOutgoing}, psdk_info::incoming::PsdkInfoIncoming, psdk_sync::incoming::PsdkSyncIncoming, psdk_terminal::incoming::PsdkTerminalIncoming, sdk_info::incoming::SdkInfoIncoming, sdk_sync::incoming::SdkSyncIncoming, sdk_tools::incoming::SdkToolsIncoming, state_message::incoming::StateMessageIncoming, ws_ping::incoming::WsPingIncoming, ClientMethodsKey
};

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
    pub fn deserialize(
        &self,
        value: &String,
    ) -> Result<Box<dyn TraitIncoming>, Box<dyn std::error::Error>> {
        let value = utils::clear_to_model_body(value)?;
        match self {
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
            ClientMethodsKey::EmulatorOpenVnc => {
                let model = serde_json::from_str::<EmulatorOpenVncIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::EmulatorRecordDisable => {
                let model = serde_json::from_str::<EmulatorRecordDisableIncoming>(&value)?;
                Ok(Box::new(model))
            }
            ClientMethodsKey::EmulatorRecordEnable => {
                let model = serde_json::from_str::<EmulatorRecordEnableIncoming>(&value)?;
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
            ClientMethodsKey::EmulatorTerminalRoot => {
                let model = serde_json::from_str::<EmulatorTerminalRootIncoming>(&value)?;
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
