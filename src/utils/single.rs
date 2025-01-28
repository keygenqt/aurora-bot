use std::sync::{LazyLock, Mutex};

use crate::service::dbus::server::ClientDbus;
use crate::service::requests::client::ClientRequest;
use crate::service::websocket::client::ClientWebsocket;

/// Singleton client requests
static CLIENT_H: LazyLock<Mutex<ClientRequest>> =
    LazyLock::new(|| Mutex::new(ClientRequest::new()));

pub fn get_request() -> Option<std::sync::MutexGuard<'static, ClientRequest>> {
    if let Ok(client) = CLIENT_H.lock() {
        Some(client)
    } else {
        None
    }
}

/// Singleton client websocket
static CLIENT_W: LazyLock<Mutex<ClientWebsocket>> =
    LazyLock::new(|| Mutex::new(ClientWebsocket::new()));

pub fn get_websocket() -> Option<std::sync::MutexGuard<'static, ClientWebsocket>> {
    if let Ok(client) = CLIENT_W.lock() {
        Some(client)
    } else {
        None
    }
}

/// Singleton client dbus
static CLIENT_D: LazyLock<Mutex<ClientDbus>> = LazyLock::new(|| Mutex::new(ClientDbus::new()));

pub fn get_dbus() -> Option<std::sync::MutexGuard<'static, ClientDbus>> {
    if let Ok(client) = CLIENT_D.lock() {
        Some(client)
    } else {
        None
    }
}
