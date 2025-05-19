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

pub fn get_tar() -> Result<String, Box<dyn std::error::Error>> {
    if let Ok(_) = exec::exec_wait_args("tar", ["--version"]) {
        return Ok("tar".into());
    }
    Err(tr!("не найдено tar"))?
}

pub fn get_sshpass() -> Result<String, Box<dyn std::error::Error>> {
    if let Ok(_) = exec::exec_wait_args("sshpass", ["-V"]) {
        return Ok("sshpass".into());
    }
    Err(tr!("не найдено sshpass"))?
}

pub fn get_aurora_bot() -> Result<String, Box<dyn std::error::Error>> {
    let path = if cfg!(debug_assertions) {
        "/home/keygenqt/Documents/Home/Projects/aurora-bot/target/debug/aurora-bot"
    } else {
        "aurora-bot"
    };
    if let Ok(_) = exec::exec_wait_args(path, ["--version"]) {
        return Ok(path.into());
    }
    if cfg!(debug_assertions) {
        println!("Run production aurora-bot!!!");
    }
    Err(tr!("не найден aurora-bot"))?
}
