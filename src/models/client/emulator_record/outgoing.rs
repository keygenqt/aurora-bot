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
}

impl EmulatorRecordOutgoing {
    pub fn new(path: String) -> Box<EmulatorRecordOutgoing> {
        Box::new(Self { path })
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
