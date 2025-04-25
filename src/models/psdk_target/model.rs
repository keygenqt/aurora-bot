use colored::Colorize;

use crate::models::TraitModel;
use crate::service::command;
use crate::tools::macros::print_info;
use crate::tools::utils;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct PsdkTargetModel {
    pub id: String,
    pub dir: String,
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
        PsdkTargetModel::get_id(&self.dir)
    }

    fn get_key(&self) -> String {
        utils::key_from_path(&self.dir)
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
    pub fn search_full(chroot: String, dir: String) -> Result<Vec<PsdkTargetModel>, Box<dyn std::error::Error>> {
        let mut models: Vec<PsdkTargetModel> = vec![];
        let mut targets: Vec<String> = vec![];
        let lines = command::psdk::psdk_targets_exec(&chroot)?;
        match utils::config_get_string(&lines, "aarch64", "─") {
            Ok(value) => targets.push(value),
            Err(_) => {}
        };
        match utils::config_get_string(&lines, "armv7hl", "─") {
            Ok(value) => targets.push(value),
            Err(_) => {}
        };
        match utils::config_get_string(&lines, "x86_64", "─") {
            Ok(value) => targets.push(value),
            Err(_) => {}
        };
        match utils::config_get_string(&lines, "i486", "─") {
            Ok(value) => targets.push(value),
            Err(_) => {}
        };
        for full_name in &targets {
            let dir = format!("{dir}/targets/{full_name}").replace("/sdks/aurora_psdk", "");
            let arch = match full_name.split("-").last() {
                Some(value) => value,
                None => continue,
            };
            if arch != "aarch64" && arch != "armv7hl" && arch != "x86_64" && arch != "i486" {
                continue;
            }
            models.push(PsdkTargetModel {
                id: PsdkTargetModel::get_id(&dir),
                dir,
                name: full_name.replace(&format!("-{arch}"), ""),
                full_name: full_name.clone(),
                arch: arch.to_string(),
            });
        }
        Ok(models)
    }
}
