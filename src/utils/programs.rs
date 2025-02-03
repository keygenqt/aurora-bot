use crate::service::exec::base;

pub fn get_vboxmanage() -> Result<String, Box<dyn std::error::Error>> {
    if let Ok(_) = base::exec_wait_args("vboxmanage", ["-v"]) {
        return Ok("vboxmanage".into());
    }
    if let Ok(_) = base::exec_wait_args("VBoxManage", ["-v"]) {
        return Ok("VBoxManage".into());
    }
    Err("не найден VBoxManage")?
}

// @todo
#[allow(dead_code)]
pub fn get_vscode() -> Result<String, Box<dyn std::error::Error>> {
    if let Ok(_) = base::exec_wait_args("code", ["--version"]) {
        return Ok("vboxmanage".into());
    }
    Err("не найден Visual Studio Code")?
}
