use std::sync::LazyLock;
use std::sync::Mutex;

use crate::service::dbus::server::ServerDbus;
use crate::service::requests::client::ClientRequest;
use crate::service::websocket::client::ClientWebsocket;

use super::macros::print_error;

/// Singleton client requests
static CLIENT_H: LazyLock<Mutex<ClientRequest>> = LazyLock::new(|| Mutex::new(ClientRequest::new(None)));

// @todo error multithread query - lock
pub fn get_request() -> std::sync::MutexGuard<'static, ClientRequest> {
    if let Ok(client) = CLIENT_H.lock() {
        client
    } else {
        print_error!("ошибка получения клиента");
        panic!("error get client")
    }
}

/// Singleton client websocket
static CLIENT_W: LazyLock<Mutex<ClientWebsocket>> = LazyLock::new(|| Mutex::new(ClientWebsocket::new()));

pub fn get_websocket() -> std::sync::MutexGuard<'static, ClientWebsocket> {
    if let Ok(client) = CLIENT_W.lock() {
        client
    } else {
        print_error!("ошибка получения websocket");
        panic!("error get websocket")
    }
}

/// Singleton client dbus
static CLIENT_D: LazyLock<Mutex<ServerDbus>> = LazyLock::new(|| Mutex::new(ServerDbus::new()));

pub fn get_dbus() -> std::sync::MutexGuard<'static, ServerDbus> {
    if let Ok(client) = CLIENT_D.lock() {
        client
    } else {
        print_error!("ошибка получения dbus");
        panic!("error get dbus")
    }
}
