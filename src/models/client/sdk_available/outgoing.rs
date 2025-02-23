use colored::Colorize;
use serde::Deserialize;
use serde::Serialize;

use crate::models::client::outgoing::DataOutgoing;
use crate::models::client::outgoing::TraitOutgoing;
use crate::tools::macros::tr;

use super::incoming::SdkAvailableIncoming;

#[derive(Serialize, Deserialize, Clone)]
pub struct SdkAvailableOutgoing {
    models: Vec<SdkAvailableItemOutgoing>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum SdkInstallType {
    SdkOnline,
    SdkOffline,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum SdkBuildType {
    BT,
    MB2,
}


#[derive(Serialize, Deserialize, Clone)]
pub struct SdkAvailableItemOutgoing {
    pub url: String,
    pub version_major: String,
    pub version_full: String,
    pub build_type: SdkBuildType,
    pub install_type: SdkInstallType,
}

impl SdkAvailableOutgoing {
    pub fn new(models: Vec<SdkAvailableItemOutgoing>) -> Box<SdkAvailableOutgoing> {
        Box::new(Self { models })
    }
}

impl TraitOutgoing for SdkAvailableOutgoing {
    fn print(&self) {
        let mut data: Vec<String> = vec![];
        for item in self.models.clone() {
            let message = tr!(
                "Аврора SDK: {}\nТип сборки: {}\nТип установки: {}\nСсылка: {}",
                item.version_full.bold().white(),
                (if item.build_type == SdkBuildType::BT { "Build Tools" } else { "MB2" }).bold().white(),
                (if item.install_type == SdkInstallType::SdkOnline { "Online" } else { "Offline" }).bold().white(),
                item.url.to_string().bright_blue(),
            );
            data.push(message);
        }
        println!("{}", data.join("\n\n"));
    }

    fn to_json(&self) -> String {
        DataOutgoing::serialize(SdkAvailableIncoming::name(), self.clone())
    }
}
