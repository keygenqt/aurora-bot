use crate::models::psdk_installed::model::PsdkInstalledModel;
use crate::models::psdk_target::model::PsdkTargetModel;
use crate::models::psdk_target_package::model::PsdkTargetPackageModel;
use crate::service::command::exec;
use crate::tools::constants;
use crate::tools::macros::tr;
use crate::tools::single;
use crate::tools::utils;
use std::fs;
use std::path::PathBuf;

pub fn psdk_targets_exec(chroot: &String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let output = exec::exec_wait_args_sudo(&chroot, ["sdk-assistant", "list", "--slow"])?;
    Ok(utils::parse_output(output.stdout))
}

pub fn target_package_install(
    chroot: &String,
    path: &PathBuf,
    target: &PsdkTargetModel,
) -> Result<(), Box<dyn std::error::Error>> {
    let output = match exec::exec_wait_args_sudo(
        &chroot,
        [
            "sb2",
            "-t",
            &target.full_name,
            "-m",
            "sdk-install",
            "-R",
            "zypper",
            "--no-gpg-checks",
            "in",
            "-y",
            &path.to_string_lossy(),
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

pub fn target_package_remove(
    chroot: &String,
    target: &PsdkTargetModel,
    package: &PsdkTargetPackageModel,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let output = match exec::exec_wait_args_sudo(
        &chroot,
        [
            "sb2",
            "-t",
            &target.full_name,
            "-m",
            "sdk-install",
            "-R",
            "zypper",
            "rm",
            "-y",
            &package.name,
        ],
    ) {
        Ok(value) => value,
        Err(e) => Err(e)?,
    };
    let lines = utils::parse_output(output.stdout);
    let mut removed = lines
        .iter()
        .filter(|e| e.contains("Removing"))
        .collect::<Vec<&String>>()
        .iter()
        .map(|e| {
            let name = e.split(" ").nth(2).unwrap().to_string();
            let name = name.replace(&package.version, "");
            let name = name.replace(&package.arch, "");
            let name = name.replace(".", "");
            name.trim_matches('-').to_string()
        })
        .collect::<Vec<String>>();
    if removed.is_empty() {
        Err("не удалось удалить пакет")?;
    }
    removed.sort();
    Ok(removed)
}

pub fn rpm_is_sign(chroot: &String, path: &PathBuf) -> bool {
    let output = match exec::exec_wait_args_sudo(&chroot, ["rpmsign-external", "verify", &path.to_string_lossy()]) {
        Ok(value) => value,
        Err(_) => return false,
    };
    let lines = utils::parse_output(output.stdout);
    !lines.is_empty() && lines.last().unwrap().contains("successfully")
}

pub fn rpm_sign(chroot: &String, path: &PathBuf) -> bool {
    let path_key = _get_regular_key();
    if path_key.is_none() {
        return false;
    }
    let path_cert = _get_regular_cert();
    if path_cert.is_none() {
        return false;
    }
    let _ = match exec::exec_wait_args_sudo(
        chroot,
        [
            "rpmsign-external",
            "sign",
            "--force",
            &format!("--key={}", path_key.unwrap().to_string_lossy()),
            &format!("--cert={}", path_cert.unwrap().to_string_lossy()),
            &path.to_string_lossy(),
        ],
    ) {
        Ok(value) => value,
        Err(_) => return false,
    };
    rpm_is_sign(chroot, path)
}

fn _get_regular_key() -> Option<PathBuf> {
    let path = utils::get_file_save_path(constants::SIGN_REG_KEY);
    if !path.exists() {
        match single::get_request().download_file(constants::SIGN_REG_KEY_URL.to_string(), |_| {}) {
            Ok(value) => match fs::rename(value, &path) {
                Ok(_) => {}
                Err(_) => return None,
            },
            Err(_) => return None,
        };
    }
    Some(path)
}

fn _get_regular_cert() -> Option<PathBuf> {
    let path = utils::get_file_save_path(constants::SIGN_REG_CERT);
    if !path.exists() {
        match single::get_request().download_file(constants::SIGN_REG_CERT_URL.to_string(), |_| {}) {
            Ok(value) => match fs::rename(value, &path) {
                Ok(_) => {}
                Err(_) => return None,
            },
            Err(_) => return None,
        };
    }
    Some(path)
}

pub fn add_sudoers_chroot_access(models: &Vec<PsdkInstalledModel>) -> Result<(), Box<dyn std::error::Error>> {
    // Get user name
    let user_name = utils::get_user_name();
    // Create files
    let mut sudoers_chroot: Vec<String> = vec![];
    let mut sudoers_mer_chroot: Vec<String> = vec![];
    for model in models {
        let psdk_dir = &model.dir;
        // Add chroot
        sudoers_chroot.push(
            (constants::PSDK_CHROOT_BODY.to_owned() + "\n")
                .replace("<user>", &user_name)
                .replace("<psdk_dir>", psdk_dir)
                .trim_start()
                .to_string(),
        );
        // Add mer chroot
        sudoers_mer_chroot.push(
            (constants::MER_PSDK_CHROOT_BODY.to_owned() + "\n")
                .replace("<user>", &user_name)
                .replace("<psdk_dir>", psdk_dir)
                .trim_start()
                .to_string(),
        );
    }
    // Join items
    let content_sudoers_chroot = sudoers_chroot.join("");
    let content_sudoers_mer_chroot = sudoers_mer_chroot.join("");
    // Add files sudoers
    utils::add_file_sudoers(constants::PSDK_CHROOT.into(), content_sudoers_chroot)?;
    utils::add_file_sudoers(constants::MER_PSDK_CHROOT.into(), content_sudoers_mer_chroot)?;
    // All ok - sudoers added
    Ok(())
}
