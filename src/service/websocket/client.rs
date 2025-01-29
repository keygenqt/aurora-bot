use std::time::Duration;

use crate::app::api::enums::SendType;
use crate::utils::constants::URL_API;
use futures_util::{SinkExt, TryStreamExt};
use reqwest::Client;
use reqwest_websocket::{Message, RequestBuilderExt, WebSocket};

use crate::{
    app::api::{
        convert::{convert_incoming, convert_outgoing},
        handler::handler_incoming,
        outgoing::models::Outgoing,
    },
    service::requests::client::ClientRequest,
    utils::{constants::WSS_API, macros::print_error, methods::print_outgoing},
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

    pub fn send(outgoing: &Outgoing) {
        let message = convert_outgoing(&outgoing).unwrap();
        tokio::spawn({
            async move {
                let request = ClientRequest::new();
                let url = format!("{URL_API}/state/connect");
                let _ = request.client.post(&url).body(message).send().await;
            }
        });
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
        let outgoing = Outgoing::connection("ping".into());
        let message = Message::Text(convert_outgoing(&outgoing).unwrap());
        websocket.send(message).await?;
        // Listen response
        while let Some(message) = websocket.try_next().await? {
            if let Message::Text(text) = message {
                match convert_incoming(text) {
                    Ok(incoming) => match handler_incoming(&incoming, SendType::Websocket).await {
                        Ok(outgoing) => match outgoing {
                            Outgoing::Connection(_) => print_outgoing(&outgoing),
                            _ => match convert_outgoing(&outgoing) {
                                Ok(outgoing) => websocket.send(Message::Text(outgoing)).await?,
                                Err(_) => print_error!("что-то пошло не так"),
                            },
                        },
                        Err(_) => print_error!("что-то пошло не так"),
                    },
                    Err(_) => print_error!("что-то пошло не так"),
                }
            }
        }
        Ok(())
    }
}
