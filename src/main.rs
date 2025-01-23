use crate::utils::macros::print_warning;
use clap::{Parser, Subcommand};
use colored::Colorize;

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
#[command(styles=utils::constants::CLAP_STYLING)]
#[command(about=format!(
    r#"

{} | {} - приложение упрощающие работу с инфраструктурой ОС Аврора.

{}"#,
    "Aurora Bot".bright_green().bold(),
    "Client".blue(),
    "Это сторонний инструмент, написанный энтузиастами!".italic()
))]
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
    /// Классическая командная строка
    Cli(feature::cli::args::CliArgs),
    /// Работа с сервисом бота
    Svc(feature::svc::args::SvcArgs),
}

#[tokio::main]
async fn main() {
    if cfg!(debug_assertions) {
        print_warning!("включен debug режим");
    }
    match App::parse().command.unwrap() {
        Commands::Cmd { command } => feature::cmd::args::run(command).await,
        Commands::Cli(arg) => feature::cli::args::run(arg).await,
        Commands::Svc(arg) => feature::svc::args::run(arg).await,
    }
}
