use std::time::Duration;

use reqwest_websocket::WebSocket;
use tokio::time::sleep;

use crate::utils::macros::print_serde;
use crate::utils::methods;

use super::incoming::models::Incoming;
use super::outgoing::models::Outgoing;

pub async fn handler_callback(value: Incoming, callback: fn(&Outgoing)) -> Option<Outgoing> {
    handler_incoming(value, Some(callback), None).await
}

pub async fn handler_websocket(value: Incoming, websocket: &mut WebSocket) -> Option<Outgoing> {
    handler_incoming(value, None, Some(websocket)).await
}

async fn handler_incoming(
    value: Incoming,
    callback: Option<fn(&Outgoing)>,
    websocket: Option<&mut WebSocket>,
) -> Option<Outgoing> {
    match value {
        Incoming::Connection(incoming) => {
            if cfg!(debug_assertions) {
                print_serde!(incoming);
            }
            Some(Outgoing::connection())
        }
        Incoming::AppInfo(incoming) => {
            if cfg!(debug_assertions) {
                print_serde!(incoming);
            }
            Some(Outgoing::app_info())
        }
        Incoming::EmulatorStart(incoming) => {
            if cfg!(debug_assertions) {
                print_serde!(incoming);
            }
            // Send state search
            let state_search = Outgoing::emulator_start_state(1);
            // Send state start
            let state_start = Outgoing::emulator_start_state(2);
            match websocket {
                Some(socket) => {
                    methods::send_state_websocket(&state_search, socket).await;
                    sleep(Duration::from_millis(1000)).await;
                    methods::send_state_websocket(&state_start, socket).await;
                    sleep(Duration::from_millis(1000)).await;
                }
                None => {
                    methods::send_state_callback(&state_search, callback);
                    sleep(Duration::from_millis(1000)).await;
                    methods::send_state_callback(&state_start, callback);
                    sleep(Duration::from_millis(1000)).await;
                }
            }
            // Send done
            Some(Outgoing::emulator_start())
        }
    }
}
