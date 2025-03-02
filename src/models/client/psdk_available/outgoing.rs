use colored::Colorize;
use serde::Deserialize;
use serde::Serialize;

use crate::models::client::outgoing::DataOutgoing;
use crate::models::client::outgoing::TraitOutgoing;
use crate::tools::macros::tr;

use super::incoming::PsdkAvailableIncoming;

#[derive(Serialize, Deserialize, Clone)]
pub struct PsdkAvailableOutgoing {
    model: PsdkAvailableItemOutgoing,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PsdkAvailableItemOutgoing {
    pub version_major: String,
    pub version_full: String,
    pub urls: Vec<String>,
}

impl PsdkAvailableOutgoing {
    pub fn new(model: PsdkAvailableItemOutgoing) -> Box<PsdkAvailableOutgoing> {
        Box::new(Self { model })
    }
}

impl TraitOutgoing for PsdkAvailableOutgoing {
    fn print(&self) {
        let mut message_lines: Vec<String> = vec![tr!("Platform SDK: {}", self.model.version_full.bold().white())];
        for url in &self.model.urls {
            if url.to_lowercase().contains("chroot") {
                message_lines.push(tr!("Chroot: {}", url.bright_blue()));
            }
            if url.to_lowercase().contains("tooling") {
                message_lines.push(tr!("Tooling: {}", url.bright_blue()));
            }
        }
        for url in &self.model.urls {
            if url.to_lowercase().contains("target") && url.contains("aarch64") {
                message_lines.push(tr!("Target (aarch64): {}", url.bright_blue()));
            }
            if url.to_lowercase().contains("target") && url.contains("armv7hl") {
                message_lines.push(tr!("Target (armv7hl): {}", url.bright_blue()));
            }
            if url.to_lowercase().contains("target") && url.contains("x86_64") {
                message_lines.push(tr!("Target (x86_64): {}", url.bright_blue()));
            }
            if url.to_lowercase().contains("target") && url.contains("i486") {
                message_lines.push(tr!("Target (i486): {}", url.bright_blue()));
            }
        }
        println!("{}", message_lines.join("\n"));
    }

    fn to_json(&self) -> String {
        DataOutgoing::serialize(PsdkAvailableIncoming::name(), self.clone())
    }
}
