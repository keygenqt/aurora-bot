use std::{
    ffi::OsStr,
    process::{Command, Output, Stdio},
};

#[allow(dead_code)]
pub fn exec_wait(program: &str) -> Result<Output, Box<dyn std::error::Error>> {
    match Command::new(program)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
    {
        Ok(output) => Ok(output),
        Err(_) => Err("команда завершилась неудачей")?,
    }
}

#[allow(dead_code)]
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
        Err(_) => Err("команда завершилась неудачей")?,
    }
}
