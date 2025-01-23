use serde::{Deserialize, Serialize};
use std::{
    io::{BufRead, BufReader},
    process::{Command, Output, Stdio},
};

#[derive(Deserialize, Serialize, Debug)]
struct AuroraCLI {
    code: u32,
    message: String,
}

/// Exec wait output
#[allow(dead_code)]
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

/// Exec read runtime output
#[allow(dead_code)]
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
    let lines = BufReader::new(child.stdout.take().unwrap()).lines();
    let mut vec = Vec::new();
    for result in lines {
        vec.push(result.unwrap());
        if vec.last().unwrap() == "}" {
            callback(vec.join("\n"));
            vec.clear();
        }
    }
}
