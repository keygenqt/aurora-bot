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
pub const CONFIGURATION_DIR: &str = ".config/aurora-bot-debug";

#[cfg(not(debug_assertions))]
pub const CONFIGURATION_DIR: &str = ".config/aurora-bot";

/// File name for save session cookie
pub const SESSION_FILE: &str = "aurora-bot.session";

/// File name for save configuration
pub const CONFIGURATION_FILE: &str = "aurora-bot.configuration";

/// File name for save devices configuration
pub const DEVICES_CONFIGURATION_FILE: &str = "devices.json";

/// File name for save environment bash
pub const ENVIRONMENT_FILE: &str = "aurora-bot.environment";

/// Files for sign rpm package
pub const SIGN_REG_KEY: &str = "regular_key.pem";
pub const SIGN_REG_CERT: &str = "regular_cert.pem";

/// Download keys for sign rpm package
pub const SIGN_REG_KEY_URL: &str = "https://developer.auroraos.ru/static/regular_key.pem";
pub const SIGN_REG_CERT_URL: &str = "https://developer.auroraos.ru/static/regular_cert.pem";

/// Version application
pub const VERSION_APP: &str = "0.1.7";

/// Version dbus api
pub const VERSION_API: &str = "0.1.0";

/// Version configuration
pub const VERSION_CONFIGURATION: &str = "1";

/// Debug log json
#[cfg(debug_assertions)]
pub const PRINT_DEBUG: bool = false;

#[cfg(not(debug_assertions))]
pub const PRINT_DEBUG: bool = false;

/// See also `clap_cargo::style::CLAP_STYLING`
pub const CLAP_STYLING: clap::builder::styling::Styles = clap::builder::styling::Styles::styled()
    .header(clap_cargo::style::HEADER)
    .usage(clap_cargo::style::USAGE)
    .literal(clap_cargo::style::LITERAL)
    .placeholder(clap_cargo::style::PLACEHOLDER)
    .error(clap_cargo::style::ERROR)
    .valid(clap_cargo::style::VALID)
    .invalid(clap_cargo::style::INVALID);

/// Sudoers data
pub const PSDK_CHROOT: &str = "psdk-chroot";
pub const PSDK_CHROOT_BODY: &str = r#"
<user> ALL=(ALL) NOPASSWD: <psdk_dir>/sdk-chroot
Defaults!<psdk_dir>/sdk-chroot env_keep += "SSH_AGENT_PID SSH_AUTH_SOCK"
"#;

pub const MER_PSDK_CHROOT: &str = "mer-psdk-chroot";
pub const MER_PSDK_CHROOT_BODY: &str = r#"
<user> ALL=(ALL) NOPASSWD: <psdk_dir>
Defaults!<psdk_dir> env_keep += "SSH_AGENT_PID SSH_AUTH_SOCK"
"#;
