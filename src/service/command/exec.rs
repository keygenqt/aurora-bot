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

// #[derive(Deserialize, Serialize)]
// struct AuroraCLI {
//     code: u32,
//     message: String,
// }

// /// Exec wait output
// #[allow(dead_code)]
// fn exec_wait() -> Result<Output, std::io::Error> {
//     Command::new("aurora-cli")
//         .args([
//             "api",
//             "--route",
//             "/tests/answer?time=1500&code=200&iterate=2",
//         ])
//         .stdout(Stdio::piped())
//         .stderr(Stdio::piped())
//         .output()
// }

// /// Exec read runtime output
// #[allow(dead_code)]
// fn exec_runtime(callback: fn(String)) {
//     let mut child = Command::new("aurora-cli")
//         .args([
//             "api",
//             "--route",
//             "/tests/answer?time=1500&code=200&iterate=2",
//         ])
//         .stdout(Stdio::piped())
//         .spawn()
//         .unwrap();
//     let lines = BufReader::new(child.stdout.take().unwrap()).lines();
//     let mut vec = Vec::new();
//     for result in lines {
//         vec.push(result.unwrap());
//         if vec.last().unwrap() == "}" {
//             callback(vec.join("\n"));
//             vec.clear();
//         }
//     }
// }
