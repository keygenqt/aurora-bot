use futures::StreamExt;
use futures::stream::FuturesUnordered;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;

use nipper::Document;
use tokio::runtime::Handle;

use crate::models::client::incoming::DataIncoming;
use crate::models::client::incoming::TraitIncoming;
use crate::models::client::selector::incoming::SelectorCmdIncoming;
use crate::service::requests::client::ClientRequest;
use crate::service::responses::common::CommonResponse;
use crate::service::responses::faq::FaqResponse;
use crate::service::responses::faq::FaqResponses;
use crate::service::responses::gitlab_tags::GitlabTagsResponse;
use crate::service::responses::user::UserResponse;
use crate::tools::constants;
use crate::tools::macros::crash;

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
    pub fn get_repo_url_sdk(&self) -> Vec<String> {
        match self.get_repo_url_files(&vec!["AuroraSDK"], None) {
            Ok(value) => value,
            Err(_) => vec![],
        }
    }

    // Get list archive files psdk
    pub fn get_repo_url_psdk(&self) -> Vec<String> {
        match self.get_repo_url_files(&vec!["PlatformSDK", "AuroraPSDK"], None) {
            Ok(value) => value,
            Err(_) => vec![],
        }
    }

    fn get_repo_url_files(
        &self,
        keys: &Vec<&str>,
        url: Option<String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
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
        let links: Vec<String> = a.iter().map(|e| e.attr("href").unwrap().to_string()).collect();
        let mut files: Vec<String> = vec![];
        for link in links {
            if link.contains("..") || link.contains("-pu-") {
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

    // Get info about Flutter from gitlab tags repo
    pub fn get_repo_tags_flutter(&self) -> Vec<GitlabTagsResponse> {
        let url = "https://gitlab.com/api/v4/projects/53055476/repository/tags?per_page=100".to_string();
        let response = match self.get_request(url) {
            Ok(response) => response,
            Err(_) => return vec![],
        };
        let body = match tokio::task::block_in_place(|| Handle::current().block_on(response.text())) {
            Ok(value) => value,
            Err(_) => return vec![],
        };
        match serde_json::from_str::<Vec<GitlabTagsResponse>>(&body) {
            Ok(value) => value,
            Err(_) => vec![],
        }
    }

    #[allow(dead_code)]
    /// Download files
    pub fn download_files<T: Fn(i32) + Send + Copy + Sync + 'static>(
        &self,
        urls: Vec<String>,
        state: T,
    ) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
        tokio::task::block_in_place(|| Handle::current().block_on(self._download_files(urls, state)))
    }

    /// Download files async
    async fn _download_files<T: Fn(i32) + Send + Copy + Sync + 'static>(
        &self,
        urls: Vec<String>,
        state: T,
    ) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
        let len = urls.len() as i32;
        let tasks = FuturesUnordered::new();
        let common_progress: &'static Mutex<i32> = Box::leak(Box::new(Mutex::new(0)));
        let save_progress: &'static Mutex<i32> = Box::leak(Box::new(Mutex::new(0)));
        // Check exist files with size
        for url in &urls {
            match self.client.get(url).send().await {
                Ok(response) => {
                    if response.content_length().is_none() {
                        Err("не удалось получить размер файла")?
                    }
                }
                Err(_) => Err("не удалось получить данные файла")?,
            };
        }
        // Send start progress
        state(0);
        // Run async downloads
        for url in urls {
            tasks.push(tokio::spawn(async move {
                match ClientRequest::new(None)
                    .download_file(url.clone(), move |_| {
                        *common_progress.lock().unwrap() += 1;
                        let value = *common_progress.lock().unwrap() / len;
                        if value < 100 && *save_progress.lock().unwrap() != value {
                            state(value)
                        }
                        *save_progress.lock().unwrap() = value;
                    })
                    .await
                {
                    Ok(value) => value,
                    Err(_) => {
                        crash!("ошибка при скачивании файла")
                    }
                }
            }));
        }
        let mut outputs = Vec::with_capacity(tasks.len());
        for task in tasks {
            outputs.push(task.await.unwrap());
        }
        state(100);
        Ok(outputs)
    }

    /// Download file
    pub async fn download_file<F: Fn(i32) + Send + Copy + Sync + 'static>(
        &self,
        url: String,
        state: F,
    ) -> Result<PathBuf, Box<dyn std::error::Error>> {
        tokio::task::block_in_place(|| Handle::current().block_on(self._download_file(url, state)))
    }

    /// Download file async
    async fn _download_file<F: Fn(i32) + Send + Copy + Sync + 'static>(
        &self,
        url: String,
        state: F,
    ) -> Result<PathBuf, Box<dyn std::error::Error>> {
        // Get name from url
        let file_name = match url.split("/").last() {
            Some(value) => value,
            None => Err("не удалось получит название файла")?,
        };
        // Create file
        let mut path = env::temp_dir();
        path.push(file_name);
        let mut file = File::create(&path)?;
        // Request
        let response = match self.client.get(url.clone()).send().await {
            Ok(response) => response,
            Err(_) => Err("не удалось получить данные файла")?,
        };
        // Get size file
        let total_size = match response.content_length() {
            Some(value) => value,
            None => Err("не удалось получить размер файла")?,
        };
        // Get stream
        let mut stream = response.bytes_stream();
        let mut save_pos: u64 = 0;
        let mut save_per: u64 = 0;
        state(0);
        while let Some(item) = stream.next().await {
            let chunk = item.or(Err(format!("Error while downloading file")))?;
            file.write_all(&chunk).or(Err(format!("Error while writing to file")))?;
            save_pos = save_pos + (chunk.len() as u64);
            let per = save_pos * 100 / total_size;
            if per != save_per {
                save_per = per;
                state(per as i32);
            }
        }
        // Result path if ok
        Ok(path)
    }
}
