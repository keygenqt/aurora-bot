use crate::service::requests::client::ClientRequest;
use crate::service::requests::response::{ApiResponse, FaqResponse, UserResponse};
use crate::utils::constants::URL_API;

/// Get data user
#[allow(dead_code)]
pub async fn get_user(
    request: Option<std::sync::MutexGuard<'static, ClientRequest>>,
) -> Result<UserResponse, Box<dyn std::error::Error>> {
    if request.is_none() {
        Err("Error load client.")?
    }
    let url = format!("{URL_API}/user/info");
    let response = match request.unwrap().get_request(url).await {
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
    request: Option<std::sync::MutexGuard<'static, ClientRequest>>,
    value: String,
) -> Result<Vec<FaqResponse>, Box<dyn std::error::Error>> {
    if request.is_none() {
        Err("Error load client.")?
    }
    let url = format!("{URL_API}/aurora-dataset/search/data/{value}");
    let response = match request.unwrap().get_request(url).await {
        Ok(response) => response,
        Err(error) => Err(error)?,
    };
    let body = match response.text().await {
        Ok(value) => value,
        Err(error) => Err(error)?,
    };
    if body.chars().nth(0).unwrap() == '[' {
        match serde_json::from_str::<Vec<FaqResponse>>(&body) {
            Ok(value) => Ok(value),
            Err(_) => match serde_json::from_str::<ApiResponse>(&body) {
                Ok(value) => Err(value.message)?,
                Err(error) => Err(error)?,
            },
        }
    } else {
        match serde_json::from_str::<FaqResponse>(&body) {
            Ok(value) => Ok(vec![value]),
            Err(_) => match serde_json::from_str::<ApiResponse>(&body) {
                Ok(value) => Err(value.message)?,
                Err(error) => Err(error)?,
            },
        }
    }
}
