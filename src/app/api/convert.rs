use crate::service::requests::response::ApiResponse;

use super::{
    enums::CommandKey,
    incoming::models::{
        ApiInfoIncoming, AppInfoIncoming, ConnectionIncoming, EmulatorStartIncoming, Incoming,
    },
    outgoing::models::Outgoing,
};

pub fn convert_incoming(value: String) -> Result<Incoming, Box<dyn std::error::Error>> {
    match value {
        // Common
        _ if value.contains(&format!("\"key\":\"{}\"", CommandKey::AppInfo.to_string())) => {
            match serde_json::from_str::<AppInfoIncoming>(&value) {
                Ok(value) => Ok(Incoming::AppInfo(value)),
                Err(error) => Err(error)?,
            }
        }
        _ if value.contains(&format!(
            "\"key\":\"{}\"",
            CommandKey::EmulatorStart.to_string()
        )) =>
        {
            match serde_json::from_str::<EmulatorStartIncoming>(&value) {
                Ok(value) => Ok(Incoming::EmulatorStart(value)),
                Err(error) => Err(error)?,
            }
        }
        // Websocket
        _ if value.contains(&format!(
            "\"key\":\"{}\"",
            CommandKey::Connection.to_string()
        )) =>
        {
            match serde_json::from_str::<ConnectionIncoming>(&value) {
                Ok(value) => Ok(Incoming::Connection(value)),
                Err(error) => Err(error)?,
            }
        }
        // D-Bus
        _ if value.contains(&format!("\"key\":\"{}\"", CommandKey::ApiInfo.to_string())) => {
            match serde_json::from_str::<ApiInfoIncoming>(&value) {
                Ok(value) => Ok(Incoming::ApiInfo(value)),
                Err(error) => Err(error)?,
            }
        }
        _ => match serde_json::from_str::<ApiResponse>(&value) {
            Ok(value) => Err(value.message)?,
            Err(error) => Err(error)?,
        },
    }
}

pub fn convert_outgoing(value: &Outgoing) -> Result<String, Box<dyn std::error::Error>> {
    match value {
        // Common
        Outgoing::Error(outgoing) => Ok(serde_json::to_string(&outgoing)?),
        Outgoing::AppInfo(outgoing) => Ok(serde_json::to_string(&outgoing)?),
        Outgoing::EmulatorStart(outgoing) => Ok(serde_json::to_string(&outgoing)?),
        Outgoing::EmulatorStartState(outgoing) => Ok(serde_json::to_string(&outgoing)?),
        // Websocket
        Outgoing::Connection(outgoing) => Ok(serde_json::to_string(&outgoing)?),
        // D-Bus
        Outgoing::ApiInfo(outgoing) => Ok(serde_json::to_string(&outgoing)?),
    }
}
