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
    models: Vec<FlutterAvailableItemOutgoing>,
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
    pub fn new(models: Vec<FlutterAvailableItemOutgoing>) -> Box<FlutterAvailableOutgoing> {
        Box::new(Self { models })
    }
}

impl TraitOutgoing for FlutterAvailableOutgoing {
    fn print(&self) {
        let mut data: Vec<String> = vec![];
        for item in self.models.clone() {
            let created_at = match DateTime::parse_from_rfc3339(&item.created_at) {
                Ok(value) => {
                    value.format("%Y-%m-%d").to_string()
                },
                Err(_) => item.created_at,
            };
            let message = tr!(
                "Flutter SDK: {}\nДата релиза: {}\nGitLab: {}\nСсылка (zip): {}\nСсылка (tar.gz): {}",
                item.version.bold().white(),
                created_at.bold().white(),
                item.url_gitlab.to_string().bright_blue(),
                item.url_zip.to_string().bright_blue(),
                item.url_tar_gz.to_string().bright_blue(),
            );
            data.push(message);
        }
        println!("{}", data.join("\n\n"));
    }

    fn to_json(&self) -> String {
        DataOutgoing::serialize(FlutterAvailableIncoming::name(), self.clone())
    }
}
