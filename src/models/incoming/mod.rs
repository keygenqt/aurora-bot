use super::outgoing::{Outgoing, OutgoingType};
use crate::models::incoming::emulator_close::EmulatorCloseIncoming;
use app_info::AppInfoIncoming;
use dbus_info::DbusInfoIncoming;
use emulator_start::EmulatorStartIncoming;
use error::ErrorIncoming;
use serde::{Deserialize, Serialize};
use ws_connect::WsConnectionIncoming;

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
    AppInfo(AppInfoIncoming),
    DbusInfo(DbusInfoIncoming),
    EmulatorClose(EmulatorCloseIncoming),
    EmulatorStart(EmulatorStartIncoming),
    Error(ErrorIncoming),
    WsConnection(WsConnectionIncoming),
}

impl Incoming {
    pub fn name(&self) -> String {
        match self {
            // Incoming: step 5
            Incoming::AppInfo(_) => AppInfoIncoming::name(),
            Incoming::DbusInfo(_) => DbusInfoIncoming::name(),
            Incoming::EmulatorClose(_) => EmulatorCloseIncoming::name(),
            Incoming::EmulatorStart(_) => EmulatorStartIncoming::name(),
            Incoming::Error(_) => ErrorIncoming::name(),
            Incoming::WsConnection(_) => WsConnectionIncoming::name(),
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
