/// Urls api
#[cfg(debug_assertions)]
pub const URL_API: &str = "http://0.0.0.0:3024/api";

#[cfg(not(debug_assertions))]
pub const URL_API: &str = "https://aurora-cos.keygenqt.com/api";

#[allow(dead_code)]
#[cfg(debug_assertions)]
pub const WSS_API: &str = "wss://aurora-cos.keygenqt.com/api/connect";

#[cfg(not(debug_assertions))]
pub const WSS_API: &str = "ws://0.0.0.0:3024/api/connect";

/// File name for save session cookie
pub const SESSION_FILE: &str = ".aurora-bot.session";

/// See also `clap_cargo::style::CLAP_STYLING`
pub const CLAP_STYLING: clap::builder::styling::Styles = clap::builder::styling::Styles::styled()
    .header(clap_cargo::style::HEADER)
    .usage(clap_cargo::style::USAGE)
    .literal(clap_cargo::style::LITERAL)
    .placeholder(clap_cargo::style::PLACEHOLDER)
    .error(clap_cargo::style::ERROR)
    .valid(clap_cargo::style::VALID)
    .invalid(clap_cargo::style::INVALID);
