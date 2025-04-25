use std::fs;
use std::path::PathBuf;

use yaml_rust::YamlLoader;

use crate::service::requests::client::ClientRequest;
use crate::tools::macros::tr;
use crate::tools::utils;

#[derive(Clone, Debug)]
pub struct PubspecModel {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub repository: Option<String>,
    pub pub_dev: Option<String>,
    pub is_plugin: bool,
    pub level: i32,
}

impl PubspecModel {
    pub fn parse_model(path_pubspec: &PathBuf) -> Result<PubspecModel, Box<dyn std::error::Error>> {
        // Load file
        let content = fs::read_to_string(path_pubspec)?;
        // Parse yaml
        let docs = YamlLoader::load_from_str(&content).unwrap();
        let doc = &docs[0];
        // Get data
        let name = match doc["name"].as_str() {
            Some(value) => value.to_string(),
            None => Err(tr!("не удалось найти поле 'name'"))?,
        };
        let version = match doc["version"].as_str() {
            Some(value) => value.to_string(),
            None => Err(tr!("не удалось найти поле 'version'"))?,
        };
        let description = match doc["description"].as_str() {
            Some(value) => Some(value.to_string()),
            None => None,
        };
        let repository = match doc["repository"].as_str() {
            Some(value) => Some(value.to_string()),
            None => None,
        };
        let mut names_dependencies: Vec<String> = vec![];
        if let Some(dependencies) = doc["dependencies"].as_hash() {
            for (key, _) in dependencies.into_iter() {
                names_dependencies.push(key.as_str().unwrap().to_string());
            }
        };
        let mut names_dev_dependencies: Vec<String> = vec![];
        if let Some(dev_dependencies) = doc["dev_dependencies"].as_hash() {
            for (key, _) in dev_dependencies.into_iter() {
                names_dev_dependencies.push(key.as_str().unwrap().to_string());
            }
        };
        let is_plugin = match doc["flutter"].as_hash() {
            Some(value) => value.iter().any(|e| e.0.as_str() == Some("plugin")),
            None => false,
        };
        Ok(PubspecModel {
            name: name.clone(),
            version,
            description,
            repository,
            pub_dev: utils::check_url(format!("https://pub.dev/packages/{}", name)),
            is_plugin,
            level: 0,
        })
    }

    pub fn search_dependencies<T: Fn(i32) + Send + Copy + Sync + 'static>(
        path_pubspec: &PathBuf,
        state: T,
    ) -> Result<Vec<PubspecModel>, Box<dyn std::error::Error>> {
        // Load file
        let content = fs::read_to_string(path_pubspec)?;
        // Parse yaml
        let docs = YamlLoader::load_from_str(&content).unwrap();
        let doc = &docs[0];
        // Get data
        let mut names_dependencies: Vec<String> = vec![];
        if let Some(dependencies) = doc["dependencies"].as_hash() {
            for (key, _) in dependencies.into_iter() {
                names_dependencies.push(key.as_str().unwrap().to_string());
            }
        };
        // Result
        Ok(ClientRequest::new(None).get_dart_packages(&names_dependencies, state)?)
    }
}
