use colored::Colorize;
use serde::Deserialize;
use serde::Serialize;

use crate::feature::outgoing::DataOutgoing;
use crate::feature::outgoing::TraitOutgoing;
use crate::tools::macros::print_success;
use crate::tools::macros::tr;

use super::incoming::DeviceScreenshotIncoming;

#[derive(Serialize, Deserialize, Clone)]
pub struct DeviceScreenshotOutgoing {
    path: String,
    base_64: Option<String>,
}

impl DeviceScreenshotOutgoing {
    pub fn new(path: String, base_64: Option<String>) -> Box<DeviceScreenshotOutgoing> {
        Box::new(Self { path, base_64 })
    }
}

impl TraitOutgoing for DeviceScreenshotOutgoing {
    fn print(&self) {
        let out = tr!("скриншот сделан: {}", self.path.blue());
        print_success!(out);
    }

    fn to_json(&self) -> String {
        DataOutgoing::serialize(DeviceScreenshotIncoming::name(), self.clone())
    }
}
