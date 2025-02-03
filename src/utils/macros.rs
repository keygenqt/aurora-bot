/// Pretty print error
macro_rules! print_error {
    ($arg:tt) => {
        println!("{}", format!("\x1b[1m\x1b[91merror\x1b[0m: {}", $arg))
    };
}
pub(crate) use print_error;

/// Pretty print info
macro_rules! print_info {
    ($arg:tt) => {
        println!("{}", format!("\x1b[1m\x1b[94minfo\x1b[0m: {}", $arg))
    };
}
pub(crate) use print_info;

/// Pretty print warning
macro_rules! print_warning {
    ($arg:tt) => {
        println!("{}", format!("\x1b[1m\x1b[93mwarning\x1b[0m: {}", $arg))
    };
}
pub(crate) use print_warning;

/// Pretty print success
macro_rules! print_success {
    ($arg:tt) => {
        println!("{}", format!("\x1b[1m\x1b[32msuccess\x1b[0m: {}", $arg))
    };
}
pub(crate) use print_success;

// @todo - for debug
// /// Pretty print json
// macro_rules! print_serde {
//     ($arg:ident) => {
//         match serde_json::to_string_pretty(&$arg) {
//             Ok(value) => println!("{}", value),
//             Err(err) => println!("{}", err),
//         }
//     };
// }
// pub(crate) use print_serde;
