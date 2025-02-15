use std::{
    thread,
    time::{self, Duration},
};

use crate::{
    models::client::{
        incoming::DataIncoming,
        outgoing::{OutgoingType, TraitOutgoing},
        ws_ping::outgoing::WsPingOutgoing,
    },
    service::requests::client::ClientRequest,
    tools::{
        constants,
        macros::{crash, print_error, print_warning},
        single,
    },
};
use futures_util::{SinkExt, TryStreamExt};
use reqwest::Client;
use reqwest_websocket::{Message, RequestBuilderExt, WebSocket};
use tokio::{runtime::Handle, time::sleep};

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
                let request = ClientRequest::new(1);
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
        let message = Message::Text(ping.to_string());
        websocket.send(message).await?;
        // Listen response
        while let Ok(Some(message)) = websocket.try_next().await {
            if let Message::Text(text) = message {
                match DataIncoming::deserialize(&text)?.deserialize(&text) {
                    Ok(incoming) => {
                        let outgoing = incoming.run(OutgoingType::Websocket);
                        // Check ping/pong
                        if outgoing.to_string().contains("WsPing") {
                            outgoing.print();
                        } else {
                            websocket.send(Message::Text(outgoing.to_string())).await?;
                        }
                    }
                    Err(_) => Err("не удалось получить incoming")?,
                }
            }
        }
        Err("соединение закрыто")?
    }
}
