use color_eyre::owo_colors::OwoColorize;
use reqwest::header;
use std::fs;
use std::sync::Arc;
use tokio::runtime::Handle;
use tokio::time::Duration;
use tokio::time::sleep;

use reqwest::Client;
use reqwest::Response;
use reqwest::StatusCode;
use reqwest_cookie_store::CookieStore;
use reqwest_cookie_store::CookieStoreMutex;

use crate::service::responses::common::CommonResponse;
use crate::tools::constants;
use crate::tools::macros::crash;
use crate::tools::macros::print_info;
use crate::tools::macros::print_success;
use crate::tools::macros::tr;
use crate::tools::utils;

pub struct ClientRequest {
    pub client: Client,
    cookie: Arc<CookieStoreMutex>,
}

impl ClientRequest {
    /// Create instance
    pub fn new(timeout: Option<u64>) -> ClientRequest {
        // Get cookie
        let cookie = match ClientRequest::load_cookie(true) {
            Ok(cookie) => std::sync::Arc::clone(&cookie),
            Err(_) => {
                crash!("ошибка чтение данных")
            }
        };
        // Get default headers
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "User-Agent",
            header::HeaderValue::from_static("AuroraBot (X11; Linux x86_64) rust/reqwest."),
        );
        // Get client
        let client = if let Some(timeout) = timeout {
            Client::builder()
                .cookie_provider(std::sync::Arc::clone(&cookie))
                .default_headers(headers)
                .timeout(Duration::from_secs(timeout))
        } else {
            Client::builder()
                .cookie_provider(std::sync::Arc::clone(&cookie))
                .default_headers(headers)
        }
        .build()
        .unwrap();
        // Done
        ClientRequest { client, cookie }
    }

    /// Load an existing set of cookies, serialized as json
    pub fn load_cookie(create: bool) -> Result<Arc<CookieStoreMutex>, Box<dyn std::error::Error>> {
        // Get path
        let path = utils::get_file_save_path(constants::SESSION_FILE);
        let buf = fs::File::open(path).map(std::io::BufReader::new);
        // Load cookie
        let cookie: CookieStore = if let Ok(file) = buf {
            CookieStore::load(file, |cookie| ::serde_json::from_str(cookie)).unwrap()
        } else {
            if create {
                CookieStore::new(None)
            } else {
                crash!("требуется авторизация")
            }
        };
        let cookie = CookieStoreMutex::new(cookie);
        // Create Arc
        Ok(Arc::new(cookie))
    }

    /// Write store to disk
    fn save_cookie(&self) {
        // Get path
        let path = utils::get_file_save_path(constants::SESSION_FILE);
        // Load file
        let mut writer = std::fs::File::create(path).map(std::io::BufWriter::new).unwrap();
        // Save cookie
        let store = self.cookie.lock().unwrap();
        store.save(&mut writer, ::serde_json::to_string).unwrap();
    }

    pub fn logout(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Get path
        let path = utils::get_file_save_path(constants::SESSION_FILE);
        fs::remove_file(path)?;
        Ok(())
    }

    /// Auth by deeplink
    fn auth(&self) -> Result<bool, Box<dyn std::error::Error>> {
        match tokio::task::block_in_place(|| Handle::current().block_on(self.auth_deeplink())) {
            Ok(value) => {
                if value.code == 200 {
                    println!(
                        "Перейдите по ссылке для авторизации: {}",
                        value.message.bright_blue().bold()
                    );
                    let token = match value.message.split("=").last() {
                        Some(value) => value,
                        None => Err(tr!("токен не найден"))?,
                    };
                    match self.auth_ping_token(String::from(token)) {
                        Ok(_) => {
                            print_success!("авторизация выполнена успешно");
                            Ok(true)
                        }
                        Err(error) => Err(error)?,
                    }
                } else {
                    Err(tr!("не удалось получить токен"))?
                }
            }
            Err(error) => Err(error)?,
        }
    }

    /// Get deeplink
    async fn auth_deeplink(&self) -> Result<CommonResponse, Box<dyn std::error::Error>> {
        let response = match self
            .client
            .get(format!("{}/auth/deeplink", constants::URL_API))
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
    pub fn auth_ping_token(&self, token: String) -> Result<(), Box<dyn std::error::Error>> {
        tokio::task::block_in_place(|| Handle::current().block_on(self.auth_ping(token, 1)))
    }

    /// Waiting for authorization
    async fn auth_ping(&self, token: String, counter: u64) -> Result<(), Box<dyn std::error::Error>> {
        if counter >= 15 {
            Err(tr!("тайм-аут подключения к серверу"))?
        }
        let response = match self
            .client
            .get(format!("{}/auth/token/{token}", constants::URL_API))
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
                    Err(tr!("это не токен"))?
                } else if value.code == 400 {
                    Err(tr!("токен устарел"))?
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

    /// Common HEAD request
    pub fn head_request(&self, url: String) -> Result<(), Box<dyn std::error::Error>> {
        tokio::task::block_in_place(|| Handle::current().block_on(self._head_request(url)))
    }

    async fn _head_request(&self, url: String) -> Result<(), Box<dyn std::error::Error>> {
        match self.client.head(&url).send().await {
            Ok(response) => {
                if StatusCode::OK == response.status() {
                    Ok(())
                } else {
                    Err(format!("Status code: {}", response.status().as_str()))?
                }
            }
            Err(error) => Err(error)?,
        }
    }

    /// Common GET request
    pub fn get_request(&self, url: String) -> Result<Response, Box<dyn std::error::Error>> {
        tokio::task::block_in_place(|| Handle::current().block_on(self._get_request(url, false)))
    }

    /// Common GET request with auth
    pub fn get_request_auth(&self, url: String) -> Result<Response, Box<dyn std::error::Error>> {
        tokio::task::block_in_place(|| Handle::current().block_on(self._get_request(url, true)))
    }

    async fn _get_request(&self, url: String, is_auth: bool) -> Result<Response, Box<dyn std::error::Error>> {
        match self.client.get(&url).send().await {
            Ok(response) => {
                if StatusCode::UNAUTHORIZED == response.status() {
                    if is_auth {
                        match self.auth() {
                            Ok(_) => match self.client.get(&url).send().await {
                                Ok(response) => Ok(response),
                                Err(error) => Err(error)?,
                            },
                            Err(error) => Err(error)?,
                        }
                    } else {
                        Err(tr!("требуется авторизация"))?
                    }
                } else {
                    Ok(response)
                }
            }
            Err(error) => Err(error)?,
        }
    }
}
