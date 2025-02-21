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

use super::incoming::EmulatorScreenshotIncoming;

#[derive(Serialize, Deserialize, Clone)]
pub struct EmulatorScreenshotOutgoing {
    path: String,
    base_64: Option<String>,
}

impl EmulatorScreenshotOutgoing {
    pub fn new(path: String) -> Box<EmulatorScreenshotOutgoing> {
        let base_64 = match fs::read(&path) {
            Ok(input) => Some(general_purpose::STANDARD.encode(input)),
            Err(_) => None,
        };
        Box::new(Self {
            path: path.clone(),
            base_64,
        })
    }
}

impl TraitOutgoing for EmulatorScreenshotOutgoing {
    fn print(&self) {
        let out = tr!("скриншот сделан: {}", self.path.blue());
        print_success!(out);
    }

    fn to_json(&self) -> String {
        DataOutgoing::serialize(EmulatorScreenshotIncoming::name(), self.clone())
    }
}
