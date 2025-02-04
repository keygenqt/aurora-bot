use app_info::OutgoingAppInfo;
use dbus_info::OutgoingDbusInfo;
use emulator_start::*;
use emulator_start_state::OutgoingEmulatorStartState;
use error::OutgoingError;
use serde::{Deserialize, Serialize};
use ws_connect::OutgoingWsConnection;

use crate::service::{dbus::server::ServerDbus, websocket::client::ClientWebsocket};

pub mod app_info;
pub mod dbus_info;
pub mod emulator_start;
pub mod emulator_start_state;
pub mod error;
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
    AppInfo(OutgoingAppInfo),
    DbusInfo(OutgoingDbusInfo),
    EmulatorStart(OutgoingEmulatorStart),
    EmulatorStartState(OutgoingEmulatorStartState),
    Error(OutgoingError),
    WsConnection(OutgoingWsConnection),
}

impl Outgoing {
    pub fn print(&self) {
        match self {
            Outgoing::AppInfo(model) => model.print(),
            Outgoing::DbusInfo(model) => model.print(),
            Outgoing::EmulatorStart(model) => model.print(),
            Outgoing::EmulatorStartState(model) => model.print(),
            Outgoing::Error(model) => model.print(),
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
