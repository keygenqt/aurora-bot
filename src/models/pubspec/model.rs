use std::fs;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;
use yaml_rust::YamlLoader;

use crate::tools::macros::tr;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PubspecModel {
    pub name: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub is_plugin: bool,
    pub models: Vec<PubspecModel>,
}

impl PubspecModel {
    pub fn search_full(path_pubspec: &PathBuf) -> Result<PubspecModel, Box<dyn std::error::Error>> {
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
        let description = match doc["description"].as_str() {
            Some(value) => value.to_string(),
            None => "".into(),
        };
        let dependencies = match doc["dependencies"].as_hash() {
            Some(value) => {
                let mut dependencies: Vec<String> = vec![];
                for (key, value) in value.into_iter() {
                    if let Some(value) = value.as_str() {
                        dependencies.push(format!("{}:{}", key.as_str().unwrap(), value));
                    }
                }
                dependencies
            },
            None => vec![],
        };
        let is_plugin = match doc["flutter"].as_hash() {
            Some(value) => {
                value.iter().any(|e| e.0.as_str() == Some("plugin"))
            },
            None => false,
        };
        // Create model
        let mut model = PubspecModel {
            name,
            description,
            dependencies,
            is_plugin,
            models: vec![]
        };
        // Search in API pub dev dependencies
        model.search_dependencies()?;
        // Result
        Ok(model)
    }

    fn search_dependencies(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.models = vec![];

        // Search models from api
        // @todo

        Ok(())
    }
}
