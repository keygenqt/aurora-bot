use colored::Colorize;
use serde::Deserialize;
use serde::Serialize;

use crate::feature::outgoing::DataOutgoing;
use crate::feature::outgoing::TraitOutgoing;
use crate::tools::macros::print_success;
use crate::tools::macros::tr;

use super::incoming::FlutterProjectReportIncoming;

#[derive(Serialize, Deserialize, Clone)]
pub struct FlutterProjectReportOutgoing {
    path: String,
    base_64: Option<String>,
}

impl FlutterProjectReportOutgoing {
    pub fn new(path: String, base_64: Option<String>) -> Box<FlutterProjectReportOutgoing> {
        Box::new(Self { path, base_64 })
    }
}

impl TraitOutgoing for FlutterProjectReportOutgoing {
    fn print(&self) {
        let out = tr!("отчет успешно сформирован: {}", self.path.blue());
        print_success!(out);
    }

    fn to_json(&self) -> String {
        DataOutgoing::serialize(FlutterProjectReportIncoming::name(), self.clone())
    }
}
