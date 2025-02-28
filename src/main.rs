use clap::Parser;
use clap::Subcommand;
use feature::cli::args::CliArgs;
use feature::svc::args::SvcArgs;
use models::client::app_info::incoming::AppInfoIncoming;
use models::client::incoming::TraitIncoming;
use models::client::outgoing::OutgoingType;
use tools::constants;
use tools::utils;

mod feature;
mod models;
mod service;
mod tools;

/// Main clap
#[derive(Parser)]
#[command(
    disable_help_flag = true,
    disable_help_subcommand = true,
    disable_version_flag = true
)]
#[command(arg_required_else_help = true)]
#[command(styles=constants::CLAP_STYLING)]
#[command(about=utils::app_about())]
struct App {
    /// Показать версию и выйти
    #[arg(short, long, default_value_t = false)]
    version: bool,
    /// Commands
    #[command(subcommand)]
    command: Option<Commands>,
    /// Показать это сообщение и выйти
    #[clap(short='h', long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,

    /// Дебаг-метод
    #[cfg(debug_assertions)]
    #[arg(short, long, default_value_t = false)]
    debug: bool,
}

/// Main groups clap
#[derive(Subcommand)]
enum Commands {
    /// Классическая командная строка
    Cli(CliArgs),
    /// Умная командная строка
    #[command(allow_external_subcommands = true)]
    #[command(arg_required_else_help = true)]
    Cmd {
        /// Команда в свободной форме
        command: Vec<String>,
        /// Показать это сообщение и выйти
        #[clap(short='h', long, action = clap::ArgAction::HelpLong)]
        help: Option<bool>,
    },

    /// Работа с Aurora Dataset
    #[command(allow_external_subcommands = true)]
    #[command(arg_required_else_help = true)]
    Faq {
        /// Вопрос в свободной форме
        search: Vec<String>,
        /// Показать это сообщение и выйти
        #[clap(short='h', long, action = clap::ArgAction::HelpLong)]
        help: Option<bool>,
    },

    /// Работа с сервисами бота
    Svc(SvcArgs),
}

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    if App::parse().debug {
        run_debug();
        return;
    }
    if App::parse().version {
        AppInfoIncoming::new().run(OutgoingType::Cli).print();
    } else {
        match App::parse().command.unwrap() {
            Commands::Cli(arg) => feature::cli::args::run(arg),
            Commands::Cmd { command, help: _ } => feature::cmd::args::run(command),
            Commands::Faq { search, help: _ } => feature::faq::args::run(search),
            Commands::Svc(arg) => feature::svc::args::run(arg),
        }
    }
}

#[cfg(debug_assertions)]
fn run_debug() {
    // method ony for debug
    // development of an easy-to-access method

    use std::time::SystemTime;

    use models::client::outgoing::TraitOutgoing;
    use models::client::state_message::outgoing::StateMessageOutgoing;
    use tools::single;

    let list = vec![
        "https://sdk-repo.omprussia.ru/sdk/installers/5.1.3/5.1.3.85-release/AuroraPSDK/Aurora_OS-5.1.3.85-MB2-Aurora_Platform_SDK_Chroot-x86_64.tar.bz2".to_string(),
        "https://sdk-repo.omprussia.ru/sdk/installers/5.1.3/5.1.3.85-release/AuroraPSDK/Aurora_OS-5.1.3.85-MB2-Aurora_SDK_Target-aarch64.tar.7z".to_string(),
        "https://sdk-repo.omprussia.ru/sdk/installers/5.1.3/5.1.3.85-release/AuroraPSDK/Aurora_OS-5.1.3.85-MB2-Aurora_SDK_Target-armv7hl.tar.7z".to_string(),
        "https://sdk-repo.omprussia.ru/sdk/installers/5.1.3/5.1.3.85-release/AuroraPSDK/Aurora_OS-5.1.3.85-MB2-Aurora_SDK_Target-x86_64.tar.7z".to_string(),
        "https://sdk-repo.omprussia.ru/sdk/installers/5.1.3/5.1.3.85-release/AuroraPSDK/Aurora_OS-5.1.3.85-MB2-Aurora_SDK_Tooling-x86_64.tar.7z".to_string(),
    ];

    let start = SystemTime::now();

    match single::get_request().download_files(list, |progress: i32| {
        // @todo progress problem
        StateMessageOutgoing::new_progress(progress.to_string()).send(&OutgoingType::Cli)
    }) {
        Ok(value) => println!("{:?}", value),
        Err(error) => panic!("{}", error),
    }

    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("it took {} seconds", duration.as_secs());
}
