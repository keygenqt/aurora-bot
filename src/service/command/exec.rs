use std::ffi::OsStr;
use std::process::Command;
use std::process::Output;
use std::process::Stdio;
use std::time::Duration;

use crate::tools::macros::tr;

#[allow(dead_code)]
pub fn exec_wait(program: &str) -> Result<Output, Box<dyn std::error::Error>> {
    match Command::new(program)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
    {
        Ok(output) => Ok(output),
        Err(_) => Err(tr!("команда завершилась неудачей"))?,
    }
}

pub fn exec_wait_args<I, S>(program: &str, args: I) -> Result<Output, Box<dyn std::error::Error>>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    match Command::new(program)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
    {
        Ok(output) => Ok(output),
        Err(_) => Err(tr!("команда завершилась неудачей"))?,
    }
}

pub fn exec_detach(program: &str, delay: u64) -> Result<(), Box<dyn std::error::Error>> {
    let mut result = false;
    if let Ok(mut child) = Command::new(program)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        std::thread::sleep(Duration::from_secs(delay));
        if let Ok(_) = child.try_wait() {
            result = true;
        } else {
            result = false;
        }
    };
    if result {
        Ok(())
    } else {
        Err(tr!("команда завершилась неудачей"))?
    }
}
