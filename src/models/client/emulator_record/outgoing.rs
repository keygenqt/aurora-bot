use std::fs;

use base64::engine::general_purpose;
use base64::Engine;
use colored::Colorize;
use serde::Deserialize;
use serde::Serialize;

use crate::models::client::outgoing::DataOutgoing;
use crate::models::client::outgoing::TraitOutgoing;
use crate::tools::macros::print_success;
use crate::tools::macros::tr;

use super::incoming::EmulatorRecordIncoming;

#[derive(Serialize, Deserialize, Clone)]
pub struct EmulatorRecordOutgoing {
    path: String,
    base_64: Option<String>,
}

impl EmulatorRecordOutgoing {
    pub fn new(path: String, gif_path: Option<&str>) -> Box<EmulatorRecordOutgoing> {
        let base_64 = if gif_path.is_some() {
            match fs::read(&gif_path.unwrap()) {
                Ok(input) => Some(general_purpose::STANDARD.encode(input)),
                Err(_) => None,
            }
        } else {
            None
        };
        Box::new(Self { path, base_64 })
    }
}

impl TraitOutgoing for EmulatorRecordOutgoing {
    fn print(&self) {
        let out = tr!("видео записано: {}", self.path.blue());
        print_success!(out);
    }

    fn to_json(&self) -> String {
        DataOutgoing::serialize(EmulatorRecordIncoming::name(), self.clone())
    }
}
