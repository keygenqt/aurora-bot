use crate::models::TraitModel;
use crate::tools::enums::PlatformArch;
use crate::tools::utils;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone)]
pub struct DemoAppModel {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub desc: String,
    pub repo: String,
    pub url: String,
}

impl DemoAppModel {
    pub fn get_id(key: &str) -> String {
        format!("{:x}", md5::compute(key.as_bytes()))
    }
}

impl TraitModel for DemoAppModel {
    fn get_id(&self) -> String {
        DemoAppModel::get_id(&self.url)
    }

    fn get_key(&self) -> String {
        utils::key_from_path(&self.url)
    }

    fn print(&self) {
        // not need
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
            let asset = match package.get_asset_platform(PlatformArch::Amd64) {
                Some(value) => value,
                None => continue,
            };
            models.push(DemoAppModel {
                id: DemoAppModel::get_id(&asset.browser_download_url),
                url: asset.browser_download_url.clone(),
                name: info.name,
                icon: info.icon,
                desc: info.desc_ru,
                repo: info.repo,
            })
        }
        Ok(models)
    }
}
