use clap::Parser;
use clap::Subcommand;
use cli::args::CliArgs;
use feature::app_info::incoming::AppInfoIncoming;
use feature::incoming::TraitIncoming;
use feature::outgoing::OutgoingType;
use svc::args::SvcArgs;
use tools::constants;
use tools::utils;

mod cli;
mod cmd;
mod faq;
mod feature;
mod models;
mod service;
mod svc;
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
            Commands::Cli(arg) => cli::args::run(arg),
            Commands::Cmd { command, help: _ } => cmd::args::run(command),
            Commands::Faq { search, help: _ } => faq::args::run(search),
            Commands::Svc(arg) => svc::args::run(arg),
        }
    }
}

#[cfg(debug_assertions)]
fn run_debug() {
    // method ony for debug
    // development of an easy-to-access method

    use std::time::SystemTime;
    let start = SystemTime::now();

    // do something

    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("it took {} seconds", duration.as_secs());
}
