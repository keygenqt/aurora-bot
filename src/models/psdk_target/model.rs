use colored::Colorize;

use crate::models::TraitModel;
use crate::tools::macros::print_info;
use crate::tools::utils;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct PsdkTargetModel {
    pub id: String,
    pub name: String,
    pub full_name: String,
    pub arch: String,
}

impl PsdkTargetModel {
    pub fn get_id(key: &str) -> String {
        format!("{:x}", md5::compute(key.as_bytes()))
    }
}

impl TraitModel for PsdkTargetModel {
    fn get_id(&self) -> String {
        PsdkTargetModel::get_id(&self.full_name)
    }

    fn get_key(&self) -> String {
        utils::key_from_path(&self.full_name)
    }

    fn print(&self) {
        let message = format!(
            "Platform Target: {}\nАрхитектура: {}",
            self.name.bold().white(),
            self.arch.to_string().bold().white(),
        );
        print_info!(message);
    }
}

impl PsdkTargetModel {
    // @todo
    // 1. search targets
    // 2. check in psdk config
    // 3. get targets in toolbox
    // 4. get targets from cache by id psdk & id target
    pub fn search_full(chroot: String) -> Result<Vec<PsdkTargetModel>, Box<dyn std::error::Error>> {
        let mut models: Vec<PsdkTargetModel> = vec![];
        // @todo
        Ok(models)
    }
}
