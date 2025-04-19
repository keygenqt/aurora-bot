use crate::models::TraitModel;
use crate::tools::enums::PlatformArch;
use crate::tools::macros::tr;
use crate::tools::utils;
use colored::Colorize;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone)]
pub struct DemoAppModel {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub desc: String,
    pub repo: String,
    pub url_aarch64: String,
    pub url_armv7hl: String,
    pub url_x86_64: String,
}

impl DemoAppModel {
    pub fn get_id(key: &str) -> String {
        format!("{:x}", md5::compute(key.as_bytes()))
    }
}

impl TraitModel for DemoAppModel {
    fn get_id(&self) -> String {
        DemoAppModel::get_id(&self.url_aarch64)
    }

    fn get_key(&self) -> String {
        self.name.clone()
    }

    fn print(&self) {
        println!(
            "{}",
            tr!(
                "{}\n{}\nРепозиторий: {}\nСсылка (aarch64): {}\nСсылка (armv7hl): {}\nСсылка (x86_64): {}",
                self.name.bold().white(),
                self.desc.white(),
                self.repo.to_string().bright_blue(),
                self.url_aarch64.to_string().bright_blue(),
                self.url_armv7hl.to_string().bright_blue(),
                self.url_x86_64.to_string().bright_blue(),
            )
        );
    }
}

impl DemoAppModel {
    pub fn search() -> Vec<DemoAppModel> {
        match Self::search_full() {
            Ok(value) => value,
            Err(_) => vec![],
        }
    }

    pub fn search_filter<T: Fn(&DemoAppModel) -> bool>(filter: T) -> Vec<DemoAppModel> {
        Self::search().iter().filter(|e| filter(e)).cloned().collect()
    }

    fn search_full() -> Result<Vec<DemoAppModel>, Box<dyn std::error::Error>> {
        let mut models: Vec<DemoAppModel> = vec![];
        let packages = utils::get_demo_apps();
        for package in packages {
            let info = match package.info.clone() {
                Some(value) => value,
                None => continue,
            };
            let asset_arm64 = match package.get_asset_platform(PlatformArch::Arm64) {
                Some(value) => value,
                None => continue,
            };
            let asset_arm32 = match package.get_asset_platform(PlatformArch::Arm32) {
                Some(value) => value,
                None => continue,
            };
            let asset_amd64 = match package.get_asset_platform(PlatformArch::Amd64) {
                Some(value) => value,
                None => continue,
            };
            models.push(DemoAppModel {
                id: DemoAppModel::get_id(&asset_arm64.browser_download_url),
                name: info.name,
                icon: info.icon,
                desc: info.desc_ru,
                repo: info.repo,
                url_aarch64: asset_arm64.browser_download_url.clone(),
                url_armv7hl: asset_arm32.browser_download_url.clone(),
                url_x86_64: asset_amd64.browser_download_url.clone(),
            })
        }
        Ok(models)
    }
}
