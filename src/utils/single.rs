use std::sync::{LazyLock, Mutex};

use crate::service::requests::client::ClientRequest;

/// Singleton client requests
static CLIENT: LazyLock<Mutex<ClientRequest>> = LazyLock::new(|| Mutex::new(ClientRequest::new()));

pub fn get_request() -> Option<std::sync::MutexGuard<'static, ClientRequest>> {
    if let Ok(client) = CLIENT.lock() {
        Some(client)
    } else {
        None
    }
}
