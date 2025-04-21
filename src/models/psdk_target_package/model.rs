use std::path::PathBuf;

use colored::Colorize;

use crate::models::TraitModel;
use crate::service::command::exec;
use crate::tools::macros::print_info;
use crate::tools::macros::tr;
use crate::tools::utils;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct PsdkTargetPackageModel {
    pub id: String,
    pub s: String,
    pub t: String,
    pub name: String,
    pub version: String,
    pub arch: String,
    pub repository: String,
}

impl PsdkTargetPackageModel {
    pub fn get_id(key: &str) -> String {
        format!("{:x}", md5::compute(key.as_bytes()))
    }
}

impl TraitModel for PsdkTargetPackageModel {
    fn get_id(&self) -> String {
        PsdkTargetPackageModel::get_id(&self.name)
    }

    fn get_key(&self) -> String {
        self.name.clone()
    }

    fn print(&self) {
        let message = format!(
            "Platform Target Package: \nName: {}\nVersion: {}\nArch: {}\nType: {}\nRepository: {}",
            self.name.bold().white(),
            self.version.bold().white(),
            self.arch.bold().white(),
            self.t.bold().white(),
            self.repository.bold().white(),
        );
        print_info!(message);
    }
}

impl PsdkTargetPackageModel {
    pub fn search_local(
        chroot: &String,
        target_name: &String,
        package: &String,
        exact: bool,
    ) -> Result<Vec<PsdkTargetPackageModel>, Box<dyn std::error::Error>> {
        let mut models: Vec<PsdkTargetPackageModel> = vec![];
        let output = match exec::exec_wait_args(
            &chroot,
            [
                "sb2",
                "-t",
                &target_name,
                "-R",
                "zypper",
                "search",
                "--installed-only",
                "-s",
                package,
            ],
        ) {
            Ok(value) => value,
            Err(e) => Err(e)?,
        };
        let lines = utils::parse_output(output.stdout);
        let packages = lines
            .iter()
            .filter(|e| e.contains(package))
            .cloned()
            .collect::<Vec<String>>();
        for package_line in packages {
            let items = package_line.split("|").map(|e| e.trim()).collect::<Vec<&str>>();
            if items.iter().count() >= 5 {
                let s = items[0];
                let name = items[1];
                let t = items[2];
                let version = items[3];
                let arch = items[4];
                let repository = items[5];
                if exact && name != package {
                    continue;
                }
                models.push(PsdkTargetPackageModel {
                    id: PsdkTargetPackageModel::get_id(&name),
                    s: s.to_string(),
                    t: t.to_string(),
                    name: name.to_string(),
                    version: version.to_string(),
                    arch: arch.to_string(),
                    repository: repository.to_string(),
                });
            }
        }
        Ok(models)
    }

    pub fn install(
        chroot: &String,
        target_name: &String,
        path: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let output = match exec::exec_wait_args(
            &chroot,
            [
                "sb2",
                "-t",
                &target_name,
                "-m",
                "sdk-install",
                "-R",
                "zypper",
                "--no-gpg-checks",
                "in",
                "-y",
                &path.to_string_lossy()
            ],
        ) {
            Ok(value) => value,
            Err(e) => Err(e)?,
        };
        let lines = utils::parse_output(output.stdout);
        if lines.iter().filter(|e| e.contains("Installing")).count() != 0 {
            Ok(())
        } else {
            Err(tr!("произошла ошибка при установке"))?
        }
    }

    pub fn remove(
        &self,
        chroot: &String,
        target_name: &String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _ = match exec::exec_wait_args(
            &chroot,
            [
                "sb2",
                "-t",
                &target_name,
                "-m",
                "sdk-install",
                "-R",
                "zypper",
                "rm",
                "-y",
                &self.name,
            ],
        ) {
            Ok(value) => value,
            Err(e) => Err(e)?,
        };
        Ok(())
    }
}
