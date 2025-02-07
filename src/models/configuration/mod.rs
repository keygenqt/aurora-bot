use super::outgoing::{Outgoing, OutgoingType};
use crate::models::incoming::emulator_close::IncomingEmulatorClose;
use app_info::IncomingAppInfo;
use dbus_info::IncomingDbusInfo;
use emulator_start::IncomingEmulatorStart;
use error::IncomingError;
use serde::{Deserialize, Serialize};
use ws_connect::IncomingWsConnection;

pub mod app_info;
pub mod dbus_info;
pub mod emulator_close;
pub mod emulator_start;
pub mod error;
pub mod ws_connect;

pub trait TraitIncoming: Clone {
    fn name() -> String;
    async fn run(&self, send_type: OutgoingType) -> Outgoing;
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Incoming {
    // Incoming: step 4
    AppInfo(IncomingAppInfo),
    DbusInfo(IncomingDbusInfo),
    EmulatorClose(IncomingEmulatorClose),
    EmulatorStart(IncomingEmulatorStart),
    Error(IncomingError),
    WsConnection(IncomingWsConnection),
}

impl Incoming {
    pub fn name(&self) -> String {
        match self {
            // Incoming: step 5
            Incoming::AppInfo(_) => IncomingAppInfo::name(),
            Incoming::DbusInfo(_) => IncomingDbusInfo::name(),
            Incoming::EmulatorClose(_) => IncomingEmulatorClose::name(),
            Incoming::EmulatorStart(_) => IncomingEmulatorStart::name(),
            Incoming::Error(_) => IncomingError::name(),
            Incoming::WsConnection(_) => IncomingWsConnection::name(),
        }
    }

    pub async fn handler(incoming: Incoming, send_type: OutgoingType) -> Outgoing {
        match incoming {
            // Incoming: step 6
            Incoming::AppInfo(model) => model.run(send_type).await,
            Incoming::DbusInfo(model) => model.run(send_type).await,
            Incoming::EmulatorClose(model) => model.run(send_type).await,
            Incoming::EmulatorStart(model) => model.run(send_type).await,
            Incoming::Error(model) => model.run(send_type).await,
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
