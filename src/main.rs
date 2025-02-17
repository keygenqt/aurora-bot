use clap::Parser;
use clap::Subcommand;
use feature::cli::args::CliArgs;
use feature::svc::args::SvcArgs;
use models::client::app_info::incoming::AppInfoIncoming;
use models::client::incoming::TraitIncoming;
use models::client::outgoing::OutgoingType;
use tools::constants;
use tools::macros::print_warning;
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

    #[command(subcommand)]
    command: Option<Commands>,

    /// Показать это сообщение и выйти
    #[clap(short='h', long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,
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
    if cfg!(debug_assertions) {
        print_warning!("включен debug режим");
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
