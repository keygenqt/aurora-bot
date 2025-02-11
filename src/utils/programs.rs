use crate::service::command::exec;

pub fn get_vboxmanage() -> Result<String, Box<dyn std::error::Error>> {
    if let Ok(_) = exec::exec_wait_args("vboxmanage", ["-v"]) {
        return Ok("vboxmanage".into());
    }
    if let Ok(_) = exec::exec_wait_args("VBoxManage", ["-v"]) {
        return Ok("VBoxManage".into());
    }
    Err("не найден VBoxManage")?
}

pub fn get_gnome_terminal() -> Result<String, Box<dyn std::error::Error>> {
    if let Ok(_) = exec::exec_wait_args("gnome-terminal", ["--version"]) {
        return Ok("gnome-terminal".into());
    }
    Err("не найден GNOME Terminal")?
}

#[allow(dead_code)]
pub fn get_vscode() -> Result<String, Box<dyn std::error::Error>> {
    if let Ok(_) = exec::exec_wait_args("code", ["--version"]) {
        return Ok("vboxmanage".into());
    }
    Err("не найден Visual Studio Code")?
}
