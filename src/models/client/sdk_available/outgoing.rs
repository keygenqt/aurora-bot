use colored::Colorize;
use serde::Deserialize;
use serde::Serialize;

use crate::models::client::outgoing::DataOutgoing;
use crate::models::client::outgoing::TraitOutgoing;
use crate::tools::macros::tr;

use super::incoming::SdkAvailableIncoming;

#[derive(Serialize, Deserialize, Clone)]
pub struct SdkAvailableOutgoing {
    model: SdkAvailableItemOutgoing,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum SdkInstallType {
    Online,
    Offline,
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
    pub fn new(model: SdkAvailableItemOutgoing) -> Box<SdkAvailableOutgoing> {
        Box::new(Self { model })
    }
}

impl SdkAvailableItemOutgoing {
    pub fn get_id(&self) -> String {
        format!("{:x}", md5::compute(format!("{}:{}:{}", self.version_full, self.name_build_type(), self.name_install_type()).as_bytes()))
    }

    pub fn name_build_type(&self) -> String {
        if self.build_type == SdkBuildType::BT {
            "Build Tools".to_string()
        } else {
            "MB2".to_string()
        }
    }

    pub fn name_install_type(&self) -> String {
        if self.install_type == SdkInstallType::Online {
            "Online".to_string()
        } else {
            "Offline".to_string()
        }
    }
}

impl TraitOutgoing for SdkAvailableOutgoing {
    fn print(&self) {
        let message = tr!(
            "Аврора SDK: {}\nТип сборки: {}\nТип установки: {}\nСсылка: {}",
            self.model.version_full.bold().white(),
            self.model.name_build_type().bold().white(),
            self.model.name_install_type().bold().white(),
            self.model.url.to_string().bright_blue(),
        );
        println!("{}", message);
    }

    fn to_json(&self) -> String {
        DataOutgoing::serialize(SdkAvailableIncoming::name(), self.clone())
    }
}
