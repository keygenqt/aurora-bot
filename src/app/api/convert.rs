use crate::service::requests::response::ApiResponse;

use super::{
    enums::ClientKey,
    incoming::models::{AppInfoIncoming, ConnectionIncoming, EmulatorStartIncoming, Incoming},
    outgoing::models::Outgoing,
};

pub fn convert_incoming(value: String) -> Result<Incoming, Box<dyn std::error::Error>> {
    match value {
        _ if value.contains(&format!(
            "\"key\":\"{}\"",
            ClientKey::Connection.to_string()
        )) =>
        {
            match serde_json::from_str::<ConnectionIncoming>(&value) {
                Ok(value) => Ok(Incoming::Connection(value)),
                Err(error) => Err(error)?,
            }
        }
        _ if value.contains(&format!("\"key\":\"{}\"", ClientKey::AppInfo.to_string())) => {
            match serde_json::from_str::<AppInfoIncoming>(&value) {
                Ok(value) => Ok(Incoming::AppInfo(value)),
                Err(error) => Err(error)?,
            }
        }
        _ if value.contains(&format!(
            "\"key\":\"{}\"",
            ClientKey::EmulatorStart.to_string()
        )) =>
        {
            match serde_json::from_str::<EmulatorStartIncoming>(&value) {
                Ok(value) => Ok(Incoming::EmulatorStart(value)),
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
        Outgoing::AppInfo(outgoing) => Ok(serde_json::to_string(&outgoing)?),
        Outgoing::EmulatorStart(outgoing) => Ok(serde_json::to_string(&outgoing)?),
        Outgoing::EmulatorStartState(outgoing) => Ok(serde_json::to_string(&outgoing)?),
        _ => Err("not found model")?,
    }
}
