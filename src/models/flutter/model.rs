use crate::models::configuration::Config;
use crate::utils::methods;
use serde::{Deserialize, Serialize};
use std::time::{Instant};

#[derive(Serialize, Deserialize, Clone)]
pub struct FlutterModel {
    pub path: String,
    pub version: String,
}

impl FlutterModel {
    #[allow(dead_code)]
    pub async fn search() -> Result<Vec<FlutterModel>, Box<dyn std::error::Error>> {
        match Config::load_flutters() {
            None => Self::search_full().await,
            Some(config) => Ok(config.iter().map(|e| e.to_model()).collect()),
        }
    }

    pub async fn search_full() -> Result<Vec<FlutterModel>, Box<dyn std::error::Error>> {
        // @todo search flutter by system
        let start = Instant::now();
        let flutters_path = methods::search_files("bin/flutter");
        println!("{:?}", flutters_path);
        let duration = start.elapsed();
        println!("Time elapsed is: {:?}", duration);
        Ok(vec![])
    }
}
