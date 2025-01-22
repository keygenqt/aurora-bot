use std::{path::Path, sync::Arc};

use color_eyre::owo_colors::OwoColorize;
use tokio::time::{sleep, Duration};

use reqwest::Client;
use reqwest_cookie_store::{CookieStore, CookieStoreMutex};
use serde::{Deserialize, Serialize};

// const URL_API: &str = "https://aurora-cos.keygenqt.com/api";
// const WSS_API: &str = "wss://aurora-cos.keygenqt.com/api/connect";

const URL_API_DEBUG: &str = "http://0.0.0.0:3024/api";
// const WSS_API_DEBUG: &str = "ws://0.0.0.0:3024/api/connect";

const SESSION_FILE: &str = "aurora-bot.session";

#[derive(Deserialize, Serialize, Debug)]
pub struct ApiResponse {
    code: u32,
    message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserResponse {
    #[serde(alias = "tgId")]
    tg_id: u64,
    #[serde(alias = "firstName")]
    first_name: String,
    #[serde(alias = "lastName")]
    last_name: String,
    #[serde(alias = "userName")]
    user_name: String,
    #[serde(alias = "mode")]
    mode: String,
}

pub struct ClientRequest {
    client: Client,
    cookie: Arc<CookieStoreMutex>,
}

impl ClientRequest {
    pub fn new() -> ClientRequest {
        let cookie = ClientRequest::load_cookie();
        let client = Client::builder()
            .cookie_provider(std::sync::Arc::clone(&cookie))
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();
        return ClientRequest { client, cookie };
    }

    // Load an existing set of cookies, serialized as json
    fn load_cookie() -> Arc<CookieStoreMutex> {
        // Get path
        let home = std::env::var("HOME").unwrap();
        let path = Path::new(&home).join(SESSION_FILE);
        let buf = std::fs::File::open(path).map(std::io::BufReader::new);
        // Load cookie
        let cookie: CookieStore = if let Ok(file) = buf {
            CookieStore::load(file, |cookie| ::serde_json::from_str(cookie)).unwrap()
        } else {
            CookieStore::new(None)
        };
        let cookie = CookieStoreMutex::new(cookie);
        // Create Arc
        return std::sync::Arc::new(cookie);
    }

    // Write store to disk
    fn save_cookie(&self) {
        // Get path
        let home = std::env::var("HOME").unwrap();
        let path = Path::new(&home).join(SESSION_FILE);
        // Load file
        let mut writer = std::fs::File::create(path)
            .map(std::io::BufWriter::new)
            .unwrap();
        // Save cookie
        let store = self.cookie.lock().unwrap();
        store.save(&mut writer, ::serde_json::to_string).unwrap();
    }

    pub async fn auth(&self, callback: fn(&String)) -> Result<bool, Box<dyn std::error::Error>> {
        let is_auth = match self.get_user().await {
            Ok(_) => true,
            Err(_) => false,
        };
        if !is_auth {
            match self.auth_deeplink().await {
                Ok(value) => {
                    if value.code == 200 {
                        callback(&value.message);
                        let token = match value.message.split("=").last() {
                            Some(value) => value,
                            None => Err("Not found token")?,
                        };
                        match self.auth_ping(token).await {
                            Ok(_) => {
                                self.save_cookie();
                                Ok(true)
                            }
                            Err(error) => Err(error)?,
                        }
                    } else {
                        Err("Error get token.")?
                    }
                }
                Err(error) => Err(error)?,
            }
        } else {
            Ok(true)
        }
    }

    pub async fn auth_deeplink(&self) -> Result<ApiResponse, Box<dyn std::error::Error>> {
        let response = match self
            .client
            .get(format!("{URL_API_DEBUG}/auth/deeplink"))
            .send()
            .await
        {
            Ok(value) => value,
            Err(error) => Err(error)?,
        };
        let body = match response.text().await {
            Ok(value) => value,
            Err(error) => Err(error)?,
        };
        match serde_json::from_str::<ApiResponse>(&body) {
            Ok(value) => Ok(value),
            Err(error) => Err(error)?,
        }
    }

    pub async fn auth_ping(&self, token: &str) -> Result<(), Box<dyn std::error::Error>> {
        let response = match self
            .client
            .get(format!("{URL_API_DEBUG}/auth/token/{token}"))
            .send()
            .await
        {
            Ok(value) => value,
            Err(error) => Err(error)?,
        };
        let body = match response.text().await {
            Ok(value) => value,
            Err(error) => Err(error)?,
        };
        let result = match serde_json::from_str::<ApiResponse>(&body) {
            Ok(value) => value.code == 200,
            Err(_) => false,
        };
        if !result {
            sleep(Duration::from_secs(1)).await;
            return Box::pin(self.auth_ping(token)).await;
        }
        Ok(())
    }

    pub async fn get_user(&self) -> Result<UserResponse, Box<dyn std::error::Error>> {
        let response = match self
            .client
            .get(format!("{URL_API_DEBUG}/user/info"))
            .send()
            .await
        {
            Ok(value) => value,
            Err(error) => Err(error)?,
        };
        let body = match response.text().await {
            Ok(value) => value,
            Err(error) => Err(error)?,
        };
        match serde_json::from_str::<UserResponse>(&body) {
            Ok(value) => Ok(value),
            Err(_) => match serde_json::from_str::<ApiResponse>(&body) {
                Ok(value) => Err(value.message)?,
                Err(error) => Err(error)?,
            },
        }
    }

    pub async fn get_search(&self, value: String) -> Result<String, Box<dyn std::error::Error>> {
        let response = match self
            .client
            .get(format!("{URL_API_DEBUG}/aurora-dataset/search/{value}"))
            .send()
            .await
        {
            Ok(value) => value,
            Err(error) => Err(error)?,
        };
        let body = match response.text().await {
            Ok(value) => value,
            Err(error) => Err(error)?,
        };
        let mut splitter = body.split('\n');
        let title = splitter
            .next()
            .unwrap()
            .replace(
                "<a href=\"https://t.me/m_auroraBetaBot?start=94f9061263480c4888352d1a5df520da\">",
                "",
            )
            .replace("</a>", "");
        let body = splitter.next().unwrap();
        let _ = splitter.next().unwrap();
        let _ = splitter.next().unwrap();
        let _ = splitter.next().unwrap();
        let author = splitter
            .next()
            .unwrap()
            .replace("<i>", "")
            .replace("</i>", "");

        Ok(format!(
            "=======================\n{}\n\n{}\n\n{}\n=======================",
            title.bright_cyan(),
            body.bright_white(),
            author.blue().italic()
        ))
    }
}
