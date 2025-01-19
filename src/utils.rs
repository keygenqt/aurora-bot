use std::{io::{BufReader, BufRead}, process::{Command, Output, Stdio}};
use serde::{Deserialize, Serialize};

/**
 * Just public fun utils with Result
 */
pub fn format(value: &str) -> Result<&str, std::io::Error> {
    Ok(value)
}

#[derive(Deserialize, Serialize, Debug)]
struct AuroraCLI {
    code: u32,
    message: String
}

pub fn run_tests() {
    println!("> Start exec wait...");
    match exec_wait() {
        Ok(output) =>
            println!("Result: {}", String::from_utf8_lossy(&output.stdout)),
        Err(error) =>
            println!("Error: {}", error),
    }
    println!("> Start exec runtime output...");
    exec_runtime(|result| match serde_json::from_str::<AuroraCLI>(
        result.as_str()
    ) {
        Ok(value) =>
            println!("Result: {}", serde_json::to_string_pretty(&value).unwrap()),
        Err(error) =>
            println!("> Error: {}", error),
    },)
}

/**
 * Exec wait output
 */
fn exec_wait() -> Result<Output, std::io::Error> {
    Command::new("aurora-cli")
        .args([
            "api",
            "--route",
            "/tests/answer?time=1500&code=200&iterate=2",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
}

/**
 * Exec read runtime output
 */
fn exec_runtime(callback: fn(String)) {
    let mut child = Command::new("aurora-cli")
        .args([
            "api",
            "--route",
            "/tests/answer?time=1500&code=200&iterate=2",
        ])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let lines = BufReader::new(
        child.stdout.take().unwrap()
    ).lines();
    let mut vec = Vec::new();
    for result in lines {
        vec.push(result.unwrap());
        if vec.last().unwrap() == "}" {
            callback(vec.join("\n"));
            vec.clear();
        }
    }
}
