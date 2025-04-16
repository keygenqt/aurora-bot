use std::thread;
use std::time::Duration;
use std::time::{self};

use crate::feature::incoming::DataIncoming;
use crate::feature::outgoing::OutgoingType;
use crate::feature::outgoing::TraitOutgoing;
use crate::feature::ws_ping::outgoing::WsPingOutgoing;
use crate::service::requests::client::ClientRequest;
use crate::tools::constants;
use crate::tools::macros::crash;
use crate::tools::macros::print_error;
use crate::tools::macros::print_warning;
use crate::tools::macros::tr;
use crate::tools::single;
use futures_util::SinkExt;
use futures_util::TryStreamExt;
use reqwest::Client;
use reqwest_websocket::Message;
use reqwest_websocket::RequestBuilderExt;
use reqwest_websocket::WebSocket;
use tokio::runtime::Handle;
use tokio::time::sleep;

pub struct ClientWebsocket {
    client: Client,
}

impl ClientWebsocket {
    pub fn new() -> ClientWebsocket {
        let cookie = match ClientRequest::load_cookie(false) {
            Ok(cookie) => std::sync::Arc::clone(&cookie),
            Err(error) => {
                print_error!(error);
                panic!("{}", error)
            }
        };
        let client = Client::builder()
            .cookie_provider(std::sync::Arc::clone(&cookie))
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();
        ClientWebsocket { client }
    }

    pub fn send(outgoing: String) {
        tokio::spawn({
            async move {
                let request = ClientRequest::new(Some(1));
                let url = format!("{}/state/connect", constants::URL_API);
                let _ = request.client.post(&url).body(outgoing).send().await;
            }
        });
        // Min seep for speed result
        thread::sleep(time::Duration::from_secs(1));
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        async fn _run(ws: &ClientWebsocket) -> Result<(), Box<dyn std::error::Error>> {
            match ws.connect().await {
                Ok(_) => Ok(()),
                Err(_) => Ok(ws.reconnect().await?),
            }
        }
        tokio::task::block_in_place(|| Handle::current().block_on(_run(self)))
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
        let response = match self.client.get(constants::WSS_API).upgrade().send().await {
            Ok(value) => value,
            Err(error) => Err(error)?,
        };
        // Get websocket
        let mut websocket: WebSocket = match response.into_websocket().await {
            Ok(value) => value,
            Err(_) => {
                let _ = single::get_request().logout();
                crash!("требуется авторизация")
            }
        };
        // Send connect message
        let ping = WsPingOutgoing::new();
        let message = Message::Text(ping.to_json());
        websocket.send(message).await?;
        // Listen response
        while let Ok(Some(message)) = websocket.try_next().await {
            if let Message::Text(text) = message {
                match DataIncoming::deserialize(&text)?.deserialize(&text) {
                    Ok(incoming) => {
                        let outgoing = incoming.run(OutgoingType::Websocket);
                        let json = outgoing.to_json();
                        // Check ping/pong
                        if json.contains("WsPing") {
                            outgoing.print();
                        } else {
                            websocket.send(Message::Text(json)).await?;
                        }
                    }
                    Err(_) => Err(tr!("не удалось получить incoming"))?,
                }
            }
        }
        Err(tr!("соединение закрыто"))?
    }
}
