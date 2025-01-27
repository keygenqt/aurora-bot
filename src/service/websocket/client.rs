use futures_util::{SinkExt, TryStreamExt};
use reqwest::Client;
use reqwest_websocket::{Message, RequestBuilderExt, WebSocket};
use tokio::time::Duration;

use crate::{
    app::api::{
        convert::{convert_incoming, convert_outgoing},
        enums::{ClientKey, ClientState},
        handler::handler_websocket,
        outgoing::models::ConnectionOutgoing,
    },
    service::requests::client::ClientRequest,
    utils::{constants::WSS_API, macros::print_error},
};

pub struct ClientWebsocket {
    client: Client,
}

// @todo Get execution status
impl ClientWebsocket {
    /// Create instance
    pub fn new() -> ClientWebsocket {
        // Creates a GET request with upgrades
        let client = Client::builder()
            .cookie_provider(std::sync::Arc::clone(&ClientRequest::load_cookie()))
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();
        return ClientWebsocket { client };
    }

    pub async fn connect(
        &self,
        // callback: fn(Incoming) -> Option<Outgoing>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Get response
        let response = match self.client.get(WSS_API).upgrade().send().await {
            Ok(value) => value,
            Err(error) => Err(error)?,
        };
        // Get websocket
        let mut websocket: WebSocket = match response.into_websocket().await {
            Ok(value) => value,
            Err(error) => Err(error)?,
        };
        // Send connect message
        let message = Message::Text(
            serde_json::to_string(&ConnectionOutgoing {
                key: ClientKey::Connection,
                state: ClientState::Success,
            })
            .unwrap(),
        );
        websocket.send(message).await?;
        // Listen response
        while let Some(message) = websocket.try_next().await? {
            if let Message::Text(text) = message {
                match convert_incoming(text) {
                    Ok(incoming) => match handler_websocket(incoming, &mut websocket).await {
                        Some(outgoing) => match convert_outgoing(&outgoing) {
                            Ok(outgoing) => websocket.send(Message::Text(outgoing)).await?,
                            Err(_) => {}
                        },
                        None => {}
                    },
                    Err(_) => {
                        if cfg!(debug_assertions) {
                            print_error!("ошибка отправки модели")
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
