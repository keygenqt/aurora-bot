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
}

impl EmulatorScreenshotOutgoing {
    pub fn new(path: String) -> Box<EmulatorScreenshotOutgoing> {
        Box::new(Self { path })
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
