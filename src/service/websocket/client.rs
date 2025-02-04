use std::time::Duration;

use crate::models::incoming::Incoming;
use crate::models::outgoing::ws_connect::OutgoingWsConnection;
use crate::models::outgoing::{Outgoing, OutgoingType};
use crate::print_warning;
use crate::service::requests::client::ClientRequest;
use crate::utils::constants::{URL_API, WSS_API};
use futures_util::{SinkExt, TryStreamExt};
use reqwest::Client;
use reqwest_websocket::{Message, RequestBuilderExt, WebSocket};
use tokio::time::sleep;

pub struct ClientWebsocket {
    client: Client,
}

impl ClientWebsocket {
    pub fn new() -> ClientWebsocket {
        let client = Client::builder()
            .cookie_provider(std::sync::Arc::clone(&ClientRequest::load_cookie()))
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();
        return ClientWebsocket { client };
    }

    pub fn send(outgoing: &Outgoing) {
        let message = outgoing.to_string().unwrap();
        tokio::spawn({
            async move {
                let request = ClientRequest::new();
                let url = format!("{URL_API}/state/connect");
                let _ = request.client.post(&url).body(message).send().await;
            }
        });
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self.connect().await {
            Ok(_) => Ok(()),
            Err(_) => Ok(self.reconnect().await?),
        }
    }

    pub async fn reconnect(&self) -> Result<(), Box<dyn std::error::Error>> {
        print_warning!("соединение не установлено, попытка подключения через 10с...");
        sleep(Duration::from_secs(10)).await;
        match self.connect().await {
            Ok(_) => Ok(()),
            Err(_) => Box::pin(self.reconnect()).await,
        }
    }

    async fn connect(&self) -> Result<(), Box<dyn std::error::Error>> {
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
        let outgoing = OutgoingWsConnection::new_ping();
        let message = Message::Text(outgoing.to_string().unwrap());
        websocket.send(message).await?;
        // Listen response
        while let Ok(Some(message)) = websocket.try_next().await {
            if let Message::Text(text) = message {
                match Incoming::convert(text) {
                    Ok(incoming) => {
                        match Incoming::handler(incoming, OutgoingType::Websocket).await {
                            Ok(outgoing) => match outgoing {
                                Outgoing::WsConnection(_) => outgoing.print(),
                                _ => match outgoing.to_string() {
                                    Ok(outgoing) => websocket.send(Message::Text(outgoing)).await?,
                                    Err(_) => Err("не удалось получить outgoing")?,
                                },
                            },
                            Err(_) => Err("ошибка выполнения задачи")?,
                        }
                    }
                    Err(_) => Err("не удалось получить incoming")?,
                }
            }
        }
        Err("соединение закрыто")?
    }
}
