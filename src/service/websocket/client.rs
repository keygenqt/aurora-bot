use futures_util::{SinkExt, TryStreamExt};
use reqwest::Client;
use reqwest_websocket::{Message, RequestBuilderExt};
use tokio::time::Duration;

use crate::{service::requests::client::ClientRequest, utils::constants::WSS_API};

use super::{enums::MessageKey, incoming::WebsocketIncoming, outgoing::WebsocketMessage};

pub struct ClientWebsocket {
    client: Client,
}

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
        callback: fn(WebsocketIncoming) -> WebsocketMessage,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Get response
        let response = match self.client.get(WSS_API).upgrade().send().await {
            Ok(value) => value,
            Err(error) => Err(error)?,
        };
        // Get websocket
        let mut websocket = match response.into_websocket().await {
            Ok(value) => value,
            Err(error) => Err(error)?,
        };
        // Send connect message
        let message = WebsocketMessage::connection().to_message();
        websocket.send(message).await?;
        // Listen response
        while let Some(message) = websocket.try_next().await? {
            if let Message::Text(text) = message {
                match serde_json::from_str::<WebsocketIncoming>(&text) {
                    Ok(value) => {
                        let model = callback(value);
                        if model.key != MessageKey::Empty {
                            websocket.send(model.to_message()).await?
                        }
                    }
                    Err(_) => {}
                }
            }
        }
        Ok(())
    }
}
