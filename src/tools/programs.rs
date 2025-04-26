use crate::service::command::exec;

use super::macros::tr;

pub fn get_vboxmanage() -> Result<String, Box<dyn std::error::Error>> {
    if let Ok(_) = exec::exec_wait_args("vboxmanage", ["-v"]) {
        return Ok("vboxmanage".into());
    }
    if let Ok(_) = exec::exec_wait_args("VBoxManage", ["-v"]) {
        return Ok("VBoxManage".into());
    }
    Err(tr!("не найден VBoxManage"))?
}

pub fn get_gnome_terminal() -> Result<String, Box<dyn std::error::Error>> {
    if let Ok(_) = exec::exec_wait_args("gnome-terminal", ["--version"]) {
        return Ok("gnome-terminal".into());
    }
    Err(tr!("не найден GNOME Terminal"))?
}

pub fn get_kitty_terminal() -> Result<String, Box<dyn std::error::Error>> {
    if let Ok(_) = exec::exec_wait_args("kitty", ["--version"]) {
        return Ok("kitty".into());
    }
    Err(tr!("не найден Kitty"))?
}

#[allow(dead_code)]
pub fn get_vscode() -> Result<String, Box<dyn std::error::Error>> {
    if let Ok(_) = exec::exec_wait_args("code", ["--version"]) {
        return Ok("vboxmanage".into());
    }
    Err(tr!("не найден Visual Studio Code"))?
}

pub fn get_xdg_open() -> Result<String, Box<dyn std::error::Error>> {
    if let Ok(_) = exec::exec_wait_args("xdg-open", ["--version"]) {
        return Ok("xdg-open".into());
    }
    Err(tr!("не найден xdg-open"))?
}

pub fn get_clang_format() -> Result<String, Box<dyn std::error::Error>> {
    if let Ok(_) = exec::exec_wait_args("clang-format", ["--version"]) {
        return Ok("clang-format".into());
    }
    Err(tr!("не найден clang-format"))?
}

pub fn get_sudo() -> Result<String, Box<dyn std::error::Error>> {
    if let Ok(_) = exec::exec_wait_args("sudo", ["--version"]) {
        return Ok("sudo".into());
    }
    Err(tr!("не найдено sudo"))?
}
