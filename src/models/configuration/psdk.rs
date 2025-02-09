use crate::models::configuration::Config;
use crate::models::psdk::model::PsdkModel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct PsdkConfig {
    pub dir: String,
    pub chroot: String,
    pub version: String,
    pub version_id: String,
    pub build: u8,
}

impl PsdkConfig {
    pub async fn search() -> Config {
        match PsdkModel::search_full().await {
            Ok(models) => Config::Psdk(
                models
                    .iter()
                    .map(|e| PsdkConfig {
                        dir: e.dir.clone(),
                        chroot: e.chroot.clone(),
                        version: e.version.clone(),
                        version_id: e.version_id.clone(),
                        build: e.build,
                    })
                    .collect(),
            ),
            Err(_) => Config::Psdk(vec![]),
        }
    }

    pub async fn search_force() -> Vec<PsdkModel> {
        let config = Self::search().await;
        config.clone().save();
        match config {
            Config::Psdk(models) => models.iter().map(|e| e.to_model()).collect(),
            _ => vec![],
        }
    }

    pub fn to_model(&self) -> PsdkModel {
        PsdkModel {
            dir: self.dir.clone(),
            chroot: self.chroot.clone(),
            version: self.version.clone(),
            version_id: self.version_id.clone(),
            build: self.build,
        }
    }
}
