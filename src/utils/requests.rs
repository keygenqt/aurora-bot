use std::time::Duration;

use reqwest::Client;
use serde::{Deserialize, Serialize};

const URL_API: &str = "https://aurora-cos.keygenqt.com/api";
const WSS_API: &str = "wss://aurora-cos.keygenqt.com/api/connect";

#[derive(Deserialize, Serialize, Debug)]
struct ApiResponse {
    code: u32,
    message: String
}

pub async fn http_get() {
    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    let body = client
        .get(format!("{URL_API}/user/info"))
        .timeout(Duration::from_secs(60 * 30))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    match serde_json::from_str::<ApiResponse>(&body) {
        Ok(value) => println!("{}", serde_json::to_string_pretty(&value).unwrap()),
        Err(error) => println!("> Error: {}", error),
    }
}

pub async fn http_auth() {
    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    let body = client
        .get(format!("{URL_API}/auth/deeplink"))
        .timeout(Duration::from_secs(60 * 30))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    match serde_json::from_str::<ApiResponse>(&body) {
        Ok(value) => println!("{}", serde_json::to_string_pretty(&value).unwrap()),
        Err(error) => println!("> Error: {}", error),
    }
}
