use color_eyre::owo_colors::OwoColorize;
use std::fs;
use std::{path::Path, sync::Arc};

use tokio::time::{sleep, Duration};

use reqwest::{Client, Response, StatusCode};
use reqwest_cookie_store::{CookieStore, CookieStoreMutex};

use crate::utils::macros::{print_info, print_success};

use crate::service::requests::response::{ApiResponse, FaqResponse, UserResponse};
use crate::utils::constants::{SESSION_FILE, URL_API};

pub struct ClientRequest {
    client: Client,
    cookie: Arc<CookieStoreMutex>,
}

impl ClientRequest {
    /// Create instance
    pub fn new() -> ClientRequest {
        let cookie = ClientRequest::load_cookie();
        let client = Client::builder()
            .cookie_provider(std::sync::Arc::clone(&cookie))
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();
        return ClientRequest { client, cookie };
    }

    /// Get data user
    #[allow(dead_code)]
    pub async fn get_user(&self) -> Result<UserResponse, Box<dyn std::error::Error>> {
        let url = format!("{URL_API}/user/info");
        let response = match self.client.get(url).send().await {
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

    /// Get answer
    #[allow(dead_code)]
    pub async fn get_search(
        &self,
        value: String,
    ) -> Result<FaqResponse, Box<dyn std::error::Error>> {
        let url = format!("{URL_API}/aurora-dataset/search/data/{value}");
        let response = match self.get_request(url).await {
            Ok(response) => response,
            Err(error) => Err(error)?,
        };
        let body = match response.text().await {
            Ok(value) => value,
            Err(error) => Err(error)?,
        };
        match serde_json::from_str::<FaqResponse>(&body) {
            Ok(value) => Ok(value),
            Err(_) => match serde_json::from_str::<ApiResponse>(&body) {
                Ok(value) => Err(value.message)?,
                Err(error) => Err(error)?,
            },
        }
    }

    /// Load an existing set of cookies, serialized as json
    pub fn load_cookie() -> Arc<CookieStoreMutex> {
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

    /// Write store to disk
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

    pub fn logout(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Get path
        let home = std::env::var("HOME").unwrap();
        let path = Path::new(&home).join(SESSION_FILE);
        fs::remove_file(path)?;
        Ok(())
    }

    /// Auth by deeplink
    async fn auth(&self) -> Result<bool, Box<dyn std::error::Error>> {
        match self.auth_deeplink().await {
            Ok(value) => {
                if value.code == 200 {
                    println!(
                        "Перейдите по ссылке для авторизации: {}",
                        value.message.bright_blue().bold()
                    );
                    let token = match value.message.split("=").last() {
                        Some(value) => value,
                        None => Err("токен не найден")?,
                    };
                    match self.auth_ping_token(String::from(token)).await {
                        Ok(_) => {
                            print_success!("авторизация выполнена успешно");
                            Ok(true)
                        }
                        Err(error) => Err(error)?,
                    }
                } else {
                    Err("не удалось получить токен")?
                }
            }
            Err(error) => Err(error)?,
        }
    }

    /// Get deeplink
    async fn auth_deeplink(&self) -> Result<ApiResponse, Box<dyn std::error::Error>> {
        let response = match self
            .client
            .get(format!("{URL_API}/auth/deeplink"))
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

    /// Waiting for authorization
    pub async fn auth_ping_token(&self, token: String) -> Result<(), Box<dyn std::error::Error>> {
        return self.auth_ping(token, 1).await;
    }

    /// Waiting for authorization
    async fn auth_ping(
        &self,
        token: String,
        counter: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if counter >= 15 {
            Err("тайм-аут подключения к серверу")?
        }
        let response = match self
            .client
            .get(format!("{URL_API}/auth/token/{token}"))
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
            Ok(value) => {
                if value.code == 200 {
                    true
                } else if value.code == 415 {
                    Err("это не токен")?
                } else if value.code == 400 {
                    Err("токен устарел")?
                } else {
                    false
                }
            }
            Err(_) => false,
        };
        if !result {
            if counter == 1 {
                print_info!("ожидание соединения...");
            }
            sleep(Duration::from_secs(u64::from(counter / 2))).await;
            return Box::pin(self.auth_ping(token, counter + 1)).await;
        }
        self.save_cookie();
        Ok(())
    }

    /// Common GET request with auth
    pub async fn get_request(&self, url: String) -> Result<Response, Box<dyn std::error::Error>> {
        match self.client.get(&url).send().await {
            Ok(response) => {
                if StatusCode::UNAUTHORIZED == response.status() {
                    match self.auth().await {
                        Ok(_) => match self.client.get(&url).send().await {
                            Ok(response) => Ok(response),
                            Err(error) => Err(error)?,
                        },
                        Err(error) => Err(error)?,
                    }
                } else {
                    Ok(response)
                }
            }
            Err(error) => Err(error)?,
        }
    }
}
