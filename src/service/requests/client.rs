use color_eyre::owo_colors::OwoColorize;
use std::fs;
use std::process::exit;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

use reqwest::{Client, Response, StatusCode};
use reqwest_cookie_store::{CookieStore, CookieStoreMutex};

use crate::service::responses::common::CommonResponse;
use crate::utils::macros::{print_error, print_info, print_success};

use crate::utils::constants::{SESSION_FILE, URL_API};
use crate::utils::methods::get_folder_save;

pub struct ClientRequest {
    pub client: Client,
    cookie: Arc<CookieStoreMutex>,
}

impl ClientRequest {
    /// Create instance
    pub fn new() -> ClientRequest {
        let cookie = match ClientRequest::load_cookie(true) {
            Ok(cookie) => std::sync::Arc::clone(&cookie),
            Err(error) => {
                print_error!(error);
                exit(1)
            }
        };
        let client = Client::builder()
            .cookie_provider(std::sync::Arc::clone(&cookie))
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();
        ClientRequest { client, cookie }
    }

    /// Load an existing set of cookies, serialized as json
    pub fn load_cookie(create: bool) -> Result<Arc<CookieStoreMutex>, Box<dyn std::error::Error>> {
        // Get path
        let path = get_folder_save(SESSION_FILE);
        let buf = fs::File::open(path).map(std::io::BufReader::new);
        // Load cookie
        let cookie: CookieStore = if let Ok(file) = buf {
            CookieStore::load(file, |cookie| ::serde_json::from_str(cookie)).unwrap()
        } else {
            if create {
                CookieStore::new(None)
            } else {
                Err("требуется авторизация")?
            }
        };
        let cookie = CookieStoreMutex::new(cookie);
        // Create Arc
        Ok(Arc::new(cookie))
    }

    /// Write store to disk
    fn save_cookie(&self) {
        // Get path
        let path = get_folder_save(SESSION_FILE);
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
        let path = get_folder_save(SESSION_FILE);
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
    async fn auth_deeplink(&self) -> Result<CommonResponse, Box<dyn std::error::Error>> {
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
        match serde_json::from_str::<CommonResponse>(&body) {
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
        let result = match serde_json::from_str::<CommonResponse>(&body) {
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
