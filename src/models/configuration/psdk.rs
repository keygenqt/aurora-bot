use crate::models::configuration::Config;
use crate::models::psdk_installed::model::PsdkInstalledModel;
use crate::models::psdk_target::model::PsdkTargetModel;
use crate::tools::macros::crash;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct PsdkConfig {
    pub dir: String,
    pub chroot: String,
    pub version: String,
    pub version_id: String,
    pub build: u8,
    pub home_url: String,
    pub targets: Vec<PsdkTargetModel>,
}

impl PsdkConfig {
    pub fn load_models() -> Vec<PsdkInstalledModel> {
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
        match PsdkInstalledModel::search_full() {
            Ok(models) => models
                .iter()
                .map(|e| PsdkConfig {
                    dir: e.dir.clone(),
                    chroot: e.chroot.clone(),
                    version: e.version.clone(),
                    version_id: e.version_id.clone(),
                    build: e.build,
                    home_url: e.home_url.clone(),
                    targets: e.targets.clone(),
                })
                .collect(),
            Err(error) => crash!(error),
        }
    }

    pub fn to_model(&self) -> PsdkInstalledModel {
        PsdkInstalledModel {
            id: PsdkInstalledModel::get_id(&self.chroot),
            dir: self.dir.clone(),
            chroot: self.chroot.clone(),
            version: self.version.clone(),
            version_id: self.version_id.clone(),
            build: self.build,
            home_url: self.home_url.clone(),
            targets: self.targets.clone(),
        }
    }
}
