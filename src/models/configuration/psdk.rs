use crate::models::configuration::Config;
use crate::models::psdk::model::PsdkModel;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct PsdkConfig {
    pub dir: String,
    pub chroot: String,
    pub version: String,
    pub version_id: String,
    pub build: u8,
}

impl PsdkConfig {
    pub fn load_models() -> Vec<PsdkModel> {
        let psdk = Config::load().psdk;
        if psdk.is_empty() {
            let update = Self::search();
            if Config::save_psdk(update.clone()) {
                return update.iter().map(|e| e.to_model()).collect();
            }
        }
        psdk.iter().map(|e| e.to_model()).collect()
    }

    pub fn search() -> Vec<PsdkConfig> {
        match PsdkModel::search_full() {
            Ok(models) => models
                .iter()
                .map(|e| PsdkConfig {
                    dir: e.dir.clone(),
                    chroot: e.chroot.clone(),
                    version: e.version.clone(),
                    version_id: e.version_id.clone(),
                    build: e.build,
                })
                .collect(),
            Err(_) => vec![],
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
