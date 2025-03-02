use colored::Colorize;
use serde::Deserialize;
use serde::Serialize;

use crate::models::client::outgoing::DataOutgoing;
use crate::models::client::outgoing::TraitOutgoing;
use crate::tools::macros::tr;
use chrono::DateTime;

use super::incoming::FlutterAvailableIncoming;

#[derive(Serialize, Deserialize, Clone)]
pub struct FlutterAvailableOutgoing {
    model: FlutterAvailableItemOutgoing,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FlutterAvailableItemOutgoing {
    pub tag: String,
    pub version: String,
    pub created_at: String,
    pub url_gitlab: String,
    pub url_zip: String,
    pub url_tar_gz: String,
}

impl FlutterAvailableOutgoing {
    pub fn new(model: FlutterAvailableItemOutgoing) -> Box<FlutterAvailableOutgoing> {
        Box::new(Self { model })
    }
}

impl FlutterAvailableItemOutgoing {
    pub fn get_id(&self) -> String {
        format!("{:x}", md5::compute(self.version.as_bytes()))
    }
}

impl TraitOutgoing for FlutterAvailableOutgoing {
    fn print(&self) {
        let mut data: Vec<String> = vec![];
        let created_at = match DateTime::parse_from_rfc3339(&self.model.created_at) {
            Ok(value) => value.format("%Y-%m-%d").to_string(),
            Err(_) => self.model.created_at.clone(),
        };
        let message = tr!(
            "Flutter SDK: {}\nДата релиза: {}\nGitLab: {}\nСсылка (zip): {}\nСсылка (tar.gz): {}",
            self.model.version.bold().white(),
            created_at.bold().white(),
            self.model.url_gitlab.to_string().bright_blue(),
            self.model.url_zip.to_string().bright_blue(),
            self.model.url_tar_gz.to_string().bright_blue(),
        );
        data.push(message);
        println!("{}", data.join("\n\n"));
    }

    fn to_json(&self) -> String {
        DataOutgoing::serialize(FlutterAvailableIncoming::name(), self.clone())
    }
}
