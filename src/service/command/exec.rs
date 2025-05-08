use std::ffi::OsStr;
use std::io::BufRead;
use std::io::BufReader;
use std::process::Command;
use std::process::Output;
use std::process::Stdio;
use std::time::Duration;

use crate::tools::macros::tr;
use crate::tools::programs;

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

pub fn exec_wait_args_callback<I, S, T: FnMut(String)>(
    program: &str,
    args: I,
    mut callback: T,
) -> Result<(), Box<dyn std::error::Error>>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let child = Command::new(program)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    BufReader::new(child.stderr.unwrap())
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| callback(line));

    Ok(())
}

pub fn exec_wait_args_sudo<'s>(
    program: &str,
    args: impl IntoIterator<Item = &'s str>,
) -> Result<Output, Box<dyn std::error::Error>> {
    let sudo = programs::get_sudo()?;
    let mut args_s = vec!["-n".to_string()];
    let mut args_p = vec![program.to_string()];
    let mut args_l = args.into_iter().map(|e| e.to_string()).collect::<Vec<String>>();
    args_s.append(&mut args_p);
    args_s.append(&mut args_l);
    match Command::new(sudo)
        .args(args_s)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                Ok(output)
            } else {
                if program.contains("chroot") {
                    Err(tr!(
                        "нет доступа к sudo, для работы с Platform SDK необходимо добавить sudoers"
                    ))?
                } else {
                    Err(tr!("нет доступа к запросу"))?
                }
            }
        }
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

pub fn exec_detach_args<I, S>(
    program: &str,
    args: I,
    delay: u64,
) -> Result<(), Box<dyn std::error::Error>>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut result = false;
    if let Ok(mut child) = Command::new(program)
        .args(args)
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
