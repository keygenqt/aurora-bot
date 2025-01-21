use clap::{Parser, Subcommand};

mod cli;
mod cmd;
mod svc;
mod utils;

#[derive(Debug, Parser)]
#[command(version, long_about = None)]
#[command(
    disable_help_flag = true,
    disable_help_subcommand = true,
    disable_version_flag = true
)]
#[command(arg_required_else_help = true)]
#[command(styles=utils::constants::clap_style())]
#[command(about=utils::constants::app_about())]
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

#[derive(Subcommand, Debug)]
enum Commands {
    /// Умная командная строка
    #[command(allow_external_subcommands = true)]
    Cmd { command: Option<Vec<String>> },
    /// Классическая командная строка
    Cli(cli::args::CliArgs),
    /// Работа с сервисом бота
    Svc(svc::args::SvcArgs),
}

#[tokio::main]
async fn main() {
    match App::parse().command.unwrap() {
        Commands::Cmd { command } => cmd::args::run(command),
        Commands::Cli(arg) => cli::args::run(arg).await,
        Commands::Svc(arg) => svc::args::run(arg),
    }
}
