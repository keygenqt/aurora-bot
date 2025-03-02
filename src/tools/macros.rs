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

/// Pretty print state
macro_rules! print_state {
    ($arg:tt) => {
        println!("{}", format!("\x1b[1m\x1b[36mstate\x1b[0m: {}", $arg))
    };
}
pub(crate) use print_state;

/// Pretty print debug
macro_rules! print_debug {
    ($($arg:tt)*) => {
        println!("{}", format!("\x1b[1m\x1b[93mdebug\x1b[0m: {}", format!($($arg)*)))
    }
}
pub(crate) use print_debug;

/// Text tr
macro_rules! tr {
    ($($arg:tt)*) => {
        format!($($arg)*)
    }
}
pub(crate) use tr;

/// Print error and exit
macro_rules! crash {
    ($arg:tt) => {{
        println!("{}", format!("\x1b[1m\x1b[91merror\x1b[0m: {}", $arg));
        std::process::exit(1);
    }};
}
pub(crate) use crash;
