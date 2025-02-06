use crate::models::incoming::Incoming;
use crate::service::requests::client::ClientRequest;
use crate::service::responses::common::CommonResponse;
use crate::service::responses::faq::{FaqResponse, FaqResponses};
use crate::service::responses::user::UserResponse;
use crate::utils::constants::URL_API;

impl ClientRequest {
    /// Get data user
    #[allow(dead_code)]
    pub async fn get_user(&self) -> Result<UserResponse, Box<dyn std::error::Error>> {
        let url = format!("{URL_API}/user/info");
        let response = match self.get_request(url).await {
            Ok(value) => value,
            Err(error) => Err(error)?,
        };
        let body = match response.text().await {
            Ok(value) => value,
            Err(error) => Err(error)?,
        };
        match serde_json::from_str::<UserResponse>(&body) {
            Ok(value) => Ok(value),
            Err(_) => match serde_json::from_str::<CommonResponse>(&body) {
                Ok(value) => Err(value.message)?,
                Err(error) => Err(error)?,
            },
        }
    }

    /// AI Command line
    pub async fn get_command(&self, value: String) -> Result<Incoming, Box<dyn std::error::Error>> {
        let url = format!("{URL_API}/cli-dataset/command/{value}");
        let response = match self.get_request(url).await {
            Ok(value) => value,
            Err(error) => Err(error)?,
        };
        let body = match response.text().await {
            Ok(value) => value,
            Err(error) => Err(error)?,
        };
        Incoming::convert(body)
    }

    /// Get answer
    pub async fn get_search(
        &self,
        value: String,
    ) -> Result<FaqResponses, Box<dyn std::error::Error>> {
        let url = format!("{URL_API}/aurora-dataset/search/data/{value}");
        let response = match self.get_request(url).await {
            Ok(response) => response,
            Err(error) => Err(error)?,
        };
        let body = match response.text().await {
            Ok(value) => value,
            Err(error) => Err(error)?,
        };
        match body.chars().nth(0) {
            None => Err("нет соединения")?,
            Some(char) => {
                if char == '[' {
                    match serde_json::from_str::<Vec<FaqResponse>>(&body) {
                        Ok(value) => Ok(FaqResponses(value)),
                        Err(_) => match serde_json::from_str::<CommonResponse>(&body) {
                            Ok(value) => Err(value.message)?,
                            Err(error) => Err(error)?,
                        },
                    }
                } else {
                    match serde_json::from_str::<FaqResponse>(&body) {
                        Ok(value) => Ok(FaqResponses(vec![value])),
                        Err(_) => match serde_json::from_str::<CommonResponse>(&body) {
                            Ok(value) => Err(value.message)?,
                            Err(error) => Err(error)?,
                        },
                    }
                }
            }
        }
    }
}
