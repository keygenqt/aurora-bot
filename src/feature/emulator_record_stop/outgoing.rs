use colored::Colorize;
use serde::Deserialize;
use serde::Serialize;

use crate::feature::outgoing::DataOutgoing;
use crate::feature::outgoing::TraitOutgoing;
use crate::tools::macros::print_success;
use crate::tools::macros::tr;

use super::incoming::EmulatorRecordStopIncoming;

#[derive(Serialize, Deserialize, Clone)]
pub struct EmulatorRecordStopOutgoing {
    path: String,
    base_64: Option<String>,
}

impl EmulatorRecordStopOutgoing {
    pub fn new(path: String, base_64: Option<String>) -> Box<EmulatorRecordStopOutgoing> {
        Box::new(Self { path, base_64 })
    }
}

impl TraitOutgoing for EmulatorRecordStopOutgoing {
    fn print(&self) {
        let out = tr!("видео записано: {}", self.path.blue());
        print_success!(out);
    }

    fn to_json(&self) -> String {
        DataOutgoing::serialize(EmulatorRecordStopIncoming::name(), self.clone())
    }
}
