use tokio::runtime::Handle;

use crate::models::client::incoming::{DataIncoming, TraitIncoming};
use crate::service::requests::client::ClientRequest;
use crate::service::responses::common::CommonResponse;
use crate::service::responses::faq::{FaqResponse, FaqResponses};
use crate::service::responses::user::UserResponse;
use crate::tools::constants;

impl ClientRequest {
    /// Get data user
    #[allow(dead_code)]
    pub fn get_user(&self) -> Result<UserResponse, Box<dyn std::error::Error>> {
        let url = format!("{}/user/info", constants::URL_API);
        let response = match self.get_request(url) {
            Ok(value) => value,
            Err(error) => Err(error)?,
        };
        let body = match tokio::task::block_in_place(|| Handle::current().block_on(response.text()))
        {
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
    pub fn get_command(
        &self,
        value: String,
    ) -> Result<Box<dyn TraitIncoming>, Box<dyn std::error::Error>> {
        let url = format!("{}/cli-dataset/command/{}", constants::URL_API, value);
        let response = match self.get_request(url) {
            Ok(value) => value,
            Err(error) => Err(error)?,
        };
        let body = match tokio::task::block_in_place(|| Handle::current().block_on(response.text()))
        {
            Ok(value) => value,
            Err(error) => Err(error)?,
        };
        DataIncoming::deserialize(&body)?.deserialize(&body)
    }

    #[allow(dead_code)]
    /// Get answer
    pub fn get_search(&self, value: String) -> Result<FaqResponses, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/aurora-dataset/search/data/{}",
            constants::URL_API,
            value
        );
        let response = match self.get_request(url) {
            Ok(response) => response,
            Err(error) => Err(error)?,
        };
        let body = match tokio::task::block_in_place(|| Handle::current().block_on(response.text()))
        {
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
