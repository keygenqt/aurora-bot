/// Urls api
#[cfg(debug_assertions)]
pub const URL_API: &str = "http://0.0.0.0:3024/api";

#[cfg(not(debug_assertions))]
pub const URL_API: &str = "https://aurora-cos.keygenqt.com/api";

#[cfg(debug_assertions)]
pub const WSS_API: &str = "ws://0.0.0.0:3024/api/connect";

#[cfg(not(debug_assertions))]
pub const WSS_API: &str = "wss://aurora-cos.keygenqt.com/api/connect";

/// D-Bus API name
pub const DBUS_NAME: &str = "com.keygenqt.aurora_bot";

/// File name for save session cookie
pub const SESSION_FILE: &str = ".aurora-bot.session";

/// File name for save configuration
pub const CONFIGURATION_FILE: &str = ".aurora-bot.configuration";

/// File name for save environment bash
pub const ENVIRONMENT_FILE: &str = ".aurora-bot.environment";

/// Version application
pub const VERSION_APP: &str = "0.0.4";

/// Version dbus api
pub const VERSION_API: &str = "0.0.1";

/// Version configuration
pub const VERSION_CONFIGURATION: &str = "1";

/// Debug log json
#[cfg(debug_assertions)]
pub const DEBUG_JSON: bool = false;

#[cfg(not(debug_assertions))]
pub const DEBUG_JSON: bool = false;

/// See also `clap_cargo::style::CLAP_STYLING`
pub const CLAP_STYLING: clap::builder::styling::Styles = clap::builder::styling::Styles::styled()
    .header(clap_cargo::style::HEADER)
    .usage(clap_cargo::style::USAGE)
    .literal(clap_cargo::style::LITERAL)
    .placeholder(clap_cargo::style::PLACEHOLDER)
    .error(clap_cargo::style::ERROR)
    .valid(clap_cargo::style::VALID)
    .invalid(clap_cargo::style::INVALID);
