use std::ffi::OsStr;
use std::process::Command;
use std::process::Output;
use std::process::Stdio;

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
