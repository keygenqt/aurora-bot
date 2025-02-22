use nipper::Document;
use tokio::runtime::Handle;

use crate::models::client::incoming::DataIncoming;
use crate::models::client::incoming::TraitIncoming;
use crate::models::client::selector::incoming::SelectorCmdIncoming;
use crate::service::requests::client::ClientRequest;
use crate::service::responses::common::CommonResponse;
use crate::service::responses::faq::FaqResponse;
use crate::service::responses::faq::FaqResponses;
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
        let body = match tokio::task::block_in_place(|| Handle::current().block_on(response.text())) {
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
    pub fn get_command(&self, value: String) -> Result<Box<dyn TraitIncoming>, Box<dyn std::error::Error>> {
        let url = format!("{}/cli-dataset/command/{}", constants::URL_API, value);
        let response = match self.get_request(url) {
            Ok(value) => value,
            Err(_) => Err("ошибка соединения")?,
        };
        let body = match tokio::task::block_in_place(|| Handle::current().block_on(response.text())) {
            Ok(value) => value,
            Err(_) => Err("не удалось прочитать данные ответа")?,
        };
        // Cmd selector API interface [SelectorCmdIncoming]
        if body.contains("stringData") {
            serde_json::from_str::<SelectorCmdIncoming>(&body)
                .expect("ошибка приема данных")
                .select()
        } else {
            DataIncoming::deserialize(&body)?.deserialize(&body)
        }
    }

    /// Get answer
    pub fn get_search(&self, value: String) -> Result<FaqResponses, Box<dyn std::error::Error>> {
        let url = format!("{}/aurora-dataset/search/data/{}", constants::URL_API, value);
        let response = match self.get_request(url) {
            Ok(response) => response,
            Err(error) => Err(error)?,
        };
        let body = match tokio::task::block_in_place(|| Handle::current().block_on(response.text())) {
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

    // Get list run files sdk
    pub fn get_repo_url_sdk(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        self.get_repo_url_files(&vec!["AuroraSDK"], None)
    }

    // Get list archive files psdk
    pub fn get_repo_url_psdk(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        self.get_repo_url_files(&vec!["PlatformSDK", "AuroraPSDK"], None)
    }

    fn get_repo_url_files(&self, keys: &Vec<&str>, url: Option<String>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let url_default = "https://sdk-repo.omprussia.ru/sdk/installers/".to_string();
        let url_level = match url {
            Some(value) => value,
            None => url_default,
        };
        let response = match self.get_request(url_level.clone()) {
            Ok(response) => response,
            Err(error) => Err(error)?,
        };
        let body = match tokio::task::block_in_place(|| Handle::current().block_on(response.text())) {
            Ok(value) => value,
            Err(error) => Err(error)?,
        };
        let document = Document::from(&body);
        let a = document.select("a");
        let links: Vec<String> = a.iter().map(|e| {
            e.attr("href").unwrap().to_string()
        }).collect();
        let mut files: Vec<String>  = vec![];
        for link in links {
            if link.contains("..") {
                continue;
            }
            if link.contains("exe") || link.contains("dmg") {
                continue;
            }
            if link.contains("md5sum") || link.contains("md5") {
                continue;
            }
            if !link.contains("/") {
                let file_link = format!("{}{}", url_level, link);
                for key in keys {
                    if file_link.contains(key) {
                        files.push(file_link);
                        break;
                    }
                }
                continue;
            }
            for file in self.get_repo_url_files(keys, Some(format!("{}{}", url_level, link)))? {
                files.push(file);
            }
        }
        Ok(files)
    }
}
