use colored::Colorize;
use serde::Deserialize;
use serde::Serialize;

use crate::feature::outgoing::DataOutgoing;
use crate::feature::outgoing::TraitOutgoing;
use crate::tools::macros::print_success;
use crate::tools::macros::tr;

use super::incoming::EmulatorScreenshotIncoming;

#[derive(Serialize, Deserialize, Clone)]
pub struct EmulatorScreenshotOutgoing {
    path: String,
    base_64: Option<String>,
}

impl EmulatorScreenshotOutgoing {
    pub fn new(path: String, base_64: Option<String>) -> Box<EmulatorScreenshotOutgoing> {
        Box::new(Self { path, base_64 })
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
