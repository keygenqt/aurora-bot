use clap::{Parser, Subcommand};
use models::client::{
    app_info::incoming::AppInfoIncoming, incoming::TraitIncoming, outgoing::OutgoingType,
};
use tools::{constants, utils};

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

    /// Показать это сообщение и выйти
    #[clap(short='h', long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,

    #[command(subcommand)]
    command: Option<Commands>,
}

/// Main groups clap
#[derive(Subcommand)]
enum Commands {
    /// Умная командная строка
    #[command(allow_external_subcommands = true)]
    Cmd { command: Option<Vec<String>> },
    /// Работа с Aurora Dataset
    #[command(allow_external_subcommands = true)]
    Faq { search: Option<Vec<String>> },
    /// Работа с сервисами бота
    Svc(feature::svc::args::SvcArgs),
    /// Классическая командная строка
    Cli(feature::cli::args::CliArgs),
}

#[tokio::main]
async fn main() {
    // if cfg!(debug_assertions) {
    //     print_warning!("включен debug режим");
    // }
    if App::parse().version {
        AppInfoIncoming::new().run(OutgoingType::Cli).print();
    } else {
        match App::parse().command.unwrap() {
            Commands::Cmd { command } => feature::cmd::args::run(command),
            Commands::Faq { search } => feature::faq::args::run(search),
            Commands::Svc(arg) => feature::svc::args::run(arg),
            Commands::Cli(arg) => feature::cli::args::run(arg),
        }
    }
}
