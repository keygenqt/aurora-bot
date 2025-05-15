use futures::StreamExt;
use futures::stream::FuturesUnordered;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Write;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use std::sync::Mutex;

use nipper::Document;
use tokio::runtime::Handle;

use crate::feature::incoming::DataIncoming;
use crate::feature::incoming::TraitIncoming;
use crate::feature::selector::incoming::SelectorCmdIncoming;
use crate::models::pubspec::model::PubspecModel;
use crate::service::requests::client::ClientRequest;
use crate::service::responses::common::CommonResponse;
use crate::service::responses::dart_package::DartPackageResponse;
use crate::service::responses::demo_releases::DemoAppResponse;
use crate::service::responses::demo_releases::DemoReleasesResponse;
use crate::service::responses::faq::FaqResponse;
use crate::service::responses::faq::FaqResponses;
use crate::service::responses::gitlab_tags::GitlabTagsResponse;
use crate::service::responses::user::UserResponse;
use crate::tools::constants;
use crate::tools::macros::crash;
use crate::tools::macros::tr;
use crate::tools::utils;

impl ClientRequest {
    /// Get data user
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
        let response = match self.get_request_auth(url) {
            Ok(value) => value,
            Err(_) => Err(tr!("ошибка соединения"))?,
        };
        let body = match tokio::task::block_in_place(|| Handle::current().block_on(response.text())) {
            Ok(value) => value,
            Err(_) => Err(tr!("не удалось прочитать данные ответа"))?,
        };
        // Cmd selector API interface [SelectorCmdIncoming]
        if body.contains("stringData") {
            serde_json::from_str::<SelectorCmdIncoming>(&body)
                .expect("getting data model failed")
                .select()
        } else {
            DataIncoming::deserialize(&body)?.deserialize(&body)
        }
    }

    /// Get answer
    pub fn get_search(&self, value: String) -> Result<FaqResponses, Box<dyn std::error::Error>> {
        let url = format!("{}/aurora-dataset/search/data/{}", constants::URL_API, value);
        let response = match self.get_request_auth(url) {
            Ok(response) => response,
            Err(error) => Err(error)?,
        };
        let body = match tokio::task::block_in_place(|| Handle::current().block_on(response.text())) {
            Ok(value) => value,
            Err(error) => Err(error)?,
        };
        match body.chars().nth(0) {
            None => Err(tr!("нет соединения"))?,
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

    pub fn get_repo_url_files(
        &self,
        keys: &Vec<&str>,
        url: Option<String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let url_default = "https://sdk-repo.omprussia.ru/sdk/installers/".to_string();
        let url_level = match url {
            Some(value) => value,
            None => url_default,
        };
        let response = match self.get_request_auth(url_level.clone()) {
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
        let url = "https://gitlab.com/api/v4/projects/48571227/repository/tags?per_page=100".to_string();
        let response = match self.get_request_auth(url) {
            Ok(response) => response,
            Err(_) => return vec![],
        };
        let body = match tokio::task::block_in_place(|| Handle::current().block_on(response.text())) {
            Ok(value) => value,
            Err(_) => return vec![],
        };
        let mut result = match serde_json::from_str::<Vec<GitlabTagsResponse>>(&body) {
            Ok(value) => value,
            Err(_) => vec![],
        };
        for model in &mut result {
            let version = model.name.replace("aurora", "").trim_matches('-').to_string();
            let url_repo =
                format!("https://sdk-repo.omprussia.ru/sdk/flutter/releases/flutter_aurora_{version}.tar.gz");
            model.url_repo = utils::check_url(url_repo)
        }
        return result;
    }

    // Get demos applications from repo
    pub fn get_demo_apps(&self) -> Vec<DemoReleasesResponse> {
        // Get info all packages async
        async fn get_packages_info(
            packages: Vec<DemoReleasesResponse>,
        ) -> Vec<Option<HashMap<String, Option<DemoAppResponse>>>> {
            let tasks = FuturesUnordered::new();
            for package in packages {
                tasks.push(tokio::spawn(async move {
                    let package_name = package.tag_name.split("-").next().unwrap();
                    let url = format!("https://raw.githubusercontent.com/keygenqt/aurora-apps/refs/heads/main/apps/{package_name}/spec.json");
                    let response = match ClientRequest::new(None).get_request_auth(url) {
                        Ok(response) => response,
                        Err(_) => return None,
                    };
                    let body = match response.text().await {
                        Ok(value) => value,
                        Err(_) => return None,
                    };
                    let info = match serde_json::from_str::<DemoAppResponse>(&body) {
                        Ok(value) => Some(value),
                        Err(_) => None,
                    };
                    let mut result: HashMap<String, Option<DemoAppResponse>> = HashMap::new();
                    result.insert(package_name.to_string(), info);
                    Some(result)
                }));
            }
            let mut outputs = Vec::with_capacity(tasks.len());
            for task in tasks {
                outputs.push(task.await.unwrap());
            }
            outputs
        }
        // Get packages
        let url = "https://api.github.com/repos/keygenqt/aurora-apps/releases".to_string();
        let response = match self.get_request_auth(url) {
            Ok(response) => response,
            Err(_) => return vec![],
        };
        let body = match tokio::task::block_in_place(|| Handle::current().block_on(response.text())) {
            Ok(value) => value,
            Err(_) => return vec![],
        };
        let packages = match serde_json::from_str::<Vec<DemoReleasesResponse>>(&body) {
            Ok(value) => value,
            Err(error) => {
                println!("{}", error);
                vec![]
            }
        };
        // Get result
        let mut result: Vec<DemoReleasesResponse> = vec![];
        let packages_with_info =
            tokio::task::block_in_place(|| Handle::current().block_on(get_packages_info(packages.clone())));
        for mut package in packages {
            let package_name = package.tag_name.split("-").next().unwrap();
            for list in &packages_with_info {
                let data = match list {
                    Some(value) => value,
                    None => continue,
                };
                let info = match data.get(package_name) {
                    Some(value) => value.clone(),
                    None => continue,
                };
                if info.is_none() {
                    continue;
                }
                package.info = info;
            }
            result.push(package);
        }
        // Done
        result
    }

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
            match self.client.head(url).send().await {
                Ok(_) => {}
                Err(_) => Err(tr!("не удалось получить данные файла"))?,
            };
        }
        // Send start progress
        state(0);
        // Run async downloads
        for url in urls {
            tasks.push(tokio::spawn(async move {
                match ClientRequest::new(None)
                    ._download_file(url.clone(), move |_| {
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
                    Err(error) => {
                        let message = tr!("{}", error);
                        crash!(message)
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
    pub fn download_file<F: Fn(i32) + Send + Copy + Sync + 'static>(
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
            None => Err(tr!("не удалось получит название файла"))?,
        };
        let file_name = match file_name.split("?").next() {
            Some(value) => value,
            None => Err(tr!("не удалось получит название файла"))?,
        };
        // Request
        let response = match self.client.get(url.clone()).send().await {
            Ok(response) => response,
            Err(_) => Err(tr!("не удалось получить данные файла"))?,
        };
        // Get size file
        let total_size = match response.content_length() {
            Some(value) => value,
            None => 0,
        };
        // Get path
        let mut path = env::temp_dir();
        path.push(file_name);
        // Create file
        let file = if path.exists() && total_size != 0 {
            File::open(&path)?
        } else {
            File::create(&path)?
        };
        // Check size
        let mut file = if total_size != 0 && file.metadata()?.size() == total_size {
            return Ok(path);
        } else {
            File::create(&path)?
        };
        // Get stream
        let mut stream = response.bytes_stream();
        let mut save_pos: u64 = 0;
        let mut save_per: u64 = 0;
        if total_size != 0 {
            state(0);
        }
        while let Some(item) = stream.next().await {
            let chunk = item.or(Err(format!("ошибка при загрузке файла")))?;
            file.write_all(&chunk).or(Err(format!("ошибка при записи в файл")))?;
            save_pos = save_pos + (chunk.len() as u64);
            if total_size != 0 {
                let per = save_pos * 100 / total_size;
                if per != save_per {
                    save_per = per;
                    state(per as i32);
                }
            }
        }
        // Result path if ok
        Ok(path)
    }

    /// Check url exist
    pub fn check_url(&self, url: String) -> Result<(), Box<dyn std::error::Error>> {
        match self.head_request(url) {
            Ok(_) => Ok(()),
            Err(error) => Err(error)?,
        }
    }

    /// Get dart packages by spec versions
    pub fn get_dart_packages<T: Fn(i32) + Send + Copy + Sync + 'static>(
        &self,
        names_dependencies: &Vec<String>,
        state: T,
    ) -> Result<Vec<PubspecModel>, Box<dyn std::error::Error>> {
        let mut models: Vec<PubspecModel> = vec![];
        let mut i = 0;
        state(0);
        for package_name in names_dependencies {
            i += 1;
            let per = i * 100 / names_dependencies.len();
            if per < 100 {
                state(per as i32);
            }
            let _ = self._get_dart_packages(1, package_name.clone(), &mut models);
        }
        state(100);
        Ok(models)
    }

    /// Get dart package
    pub fn _get_dart_packages(
        &self,
        level: i32,
        package_name: String,
        models: &mut Vec<PubspecModel>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if package_name == "flutter" {
            return Ok(());
        }
        if models.iter().any(|e| e.name == package_name) {
            return Ok(());
        }
        let (package, dependencies) = tokio::task::block_in_place(|| {
            Handle::current().block_on(Self::_get_dart_package(self, package_name, level))
        })?;
        // Add to array
        models.push(package);
        // Load dependencies
        if let Some(dependencies) = dependencies {
            for (key, _) in dependencies.as_object().unwrap().into_iter() {
                self._get_dart_packages(level + 1, key.as_str().to_string(), models)?;
            }
        };
        Ok(())
    }

    // Async get package
    async fn _get_dart_package(
        client: &ClientRequest,
        package_name: String,
        level: i32,
    ) -> Result<(PubspecModel, Option<serde_json::Value>), Box<dyn std::error::Error>> {
        let url = format!("https://pub.dev/api/packages/{package_name}");
        let url_pub = format!("https://pub.dev/packages/{}", package_name);
        let response = client.get_request(url)?;
        let body = response.text().await?;
        let result = serde_json::from_str::<DartPackageResponse>(&body)?;
        let is_plugin = match result.latest.pubspec.flutter {
            Some(value) => value.as_object().iter().any(|e| e.contains_key("plugin")),
            None => false,
        };
        return Ok((
            PubspecModel {
                name: result.name,
                description: result.latest.pubspec.description,
                repository: result.latest.pubspec.repository,
                version: result.latest.pubspec.version,
                pub_dev: utils::check_url(url_pub),
                is_plugin,
                level,
            },
            result.latest.pubspec.dependencies,
        ));
    }
}
