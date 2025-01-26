use crate::utils::{constants, macros::print_warning, methods};
use clap::{Parser, Subcommand};

mod feature;
mod service;
mod utils;

/// Main clap
#[derive(Debug, Parser)]
#[command(version, long_about = None)]
#[command(
    disable_help_flag = true,
    disable_help_subcommand = true,
    disable_version_flag = true
)]
#[command(arg_required_else_help = true)]
#[command(styles=constants::CLAP_STYLING)]
#[command(about=methods::app_about())]
struct App {
    /// Показать версию и выйти
    #[clap(short='v', long, action = clap::ArgAction::Version)]
    version: Option<bool>,

    /// Показать это сообщение и выйти
    #[clap(short='h', long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,

    #[command(subcommand)]
    command: Option<Commands>,
}

/// Main groups clap
#[derive(Subcommand, Debug)]
enum Commands {
    /// Умная командная строка
    #[command(allow_external_subcommands = true)]
    Cmd { command: Option<Vec<String>> },
    /// Работа с Aurora Dataset
    #[command(allow_external_subcommands = true)]
    Faq { search: Option<Vec<String>> },
    /// Работа с сервисом бота
    Svc(feature::svc::args::SvcArgs),
    // /// Классическая командная строка
    // Cli(feature::cli::args::CliArgs),
}

#[tokio::main]
async fn main() {
    if cfg!(debug_assertions) {
        print_warning!("включен debug режим");
    }
    match App::parse().command.unwrap() {
        Commands::Cmd { command } => feature::cmd::args::run(command).await,
        Commands::Faq { search } => feature::faq::args::run(search).await,
        Commands::Svc(arg) => feature::svc::args::run(arg).await,
        // Commands::Cli(arg) => feature::cli::args::run(arg).await,
    }
}
