/// Urls api
#[cfg(debug_assertions)]
pub const URL_API: &str = "http://0.0.0.0:3024/api";

#[cfg(not(debug_assertions))]
pub const URL_API: &str = "https://aurora-bot.keygenqt.com/api";

#[cfg(debug_assertions)]
pub const WSS_API: &str = "ws://0.0.0.0:3024/api/connect";

#[cfg(not(debug_assertions))]
pub const WSS_API: &str = "wss://aurora-bot.keygenqt.com/api/connect";

/// D-Bus API name
pub const DBUS_NAME: &str = "com.keygenqt.aurora_bot";

/// Folder for save config files
#[cfg(debug_assertions)]
pub const CONFIGURATION_DIR: &str = ".aurora-bot-debug";

#[cfg(not(debug_assertions))]
pub const CONFIGURATION_DIR: &str = ".aurora-bot";

/// File name for save session cookie
pub const SESSION_FILE: &str = "aurora-bot.session";

/// File name for save configuration
pub const CONFIGURATION_FILE: &str = "aurora-bot.configuration";

/// File name for save environment bash
pub const ENVIRONMENT_FILE: &str = "aurora-bot.environment";

/// Files for sign rpm package
pub const SIGN_REG_KEY: &str = "regular_key.pem";
pub const SIGN_REG_CERT: &str = "regular_cert.pem";

/// Download keys for sign rpm package
pub const SIGN_REG_KEY_URL: &str = "https://developer.auroraos.ru/static/regular_key.pem";
pub const SIGN_REG_CERT_URL: &str = "https://developer.auroraos.ru/static/regular_cert.pem";

/// Version application
pub const VERSION_APP: &str = "0.0.5";

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
