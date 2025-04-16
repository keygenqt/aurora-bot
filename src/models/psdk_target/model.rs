use colored::Colorize;

use crate::models::TraitModel;
use crate::tools::macros::print_info;
use crate::tools::utils;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone)]
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
    #[allow(dead_code)]
    #[allow(unused_mut)]
    pub fn search_full() -> Result<Vec<PsdkTargetModel>, Box<dyn std::error::Error>> {
        let mut models: Vec<PsdkTargetModel> = vec![];
        // @todo
        Ok(models)
    }
}
