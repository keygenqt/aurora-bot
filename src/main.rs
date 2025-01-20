use clap::Parser;
use clap::Subcommand;
use clap::Args;
use colored::Colorize;

fn get_about() -> String {

    let link1 = "https://developer.auroraos.ru/doc/sdk".blue().to_string();
    let link2 = "https://omprussia.gitlab.io/flutter/flutter".blue().to_string();
    let link3 = "https://developer.auroraos.ru/doc/sdk/psdk".blue().to_string();

    return format!(r#"

The application allows you to install tools for working with the Aurora OS
and simplifies working with them. More details about the tools can be found
on the documentation page:

Aurora SDK   {}
Flutter SDK  {}
Platform SDK {}

This is a third party tool written by enthusiasts!"#, link1, link2, link3);
}

pub fn get_styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .usage(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::White))),
        )
        .header(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
        .literal(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::BrightCyan))),
        )
        .invalid(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
        .error(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
        .valid(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
        .placeholder(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow))),
        )
}

/// A fictional versioning CLI
#[derive(Debug, Parser)] // requires `derive` feature
#[command(about = get_about(), long_about = None)]
#[command(args_conflicts_with_subcommands = true)]
#[command(disable_help_flag = true)]
#[command(styles=get_styles())]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Specify config path.
    #[arg(short='c', long)]
    config: Option<String>,

    /// Clear cached data.
    #[arg(short='r', long, default_value_t = false)]
    remove_cache: bool,

    /// Show the version and exit.
    #[arg(short='v', long, default_value_t = false)]
    version: bool,

    /// Show this message and exit.
    #[clap(short='h', long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,
}

// /// Aurora Bot | Client - приложение обеспечивающие работу с инфраструктурой ОС Аврора, связь с сервером, реализующем умную командную строку и удалённое взаимодействие 😎
// #[derive(Parser, Debug)]
// #[command(version)]
// struct Args {
//     /// Just tell me what to do
//     #[arg(short, long)]
//     ai: String,

//     /// Search data for Aurora Dataset
//     #[arg(short, long)]
//     search: String,

//     // /// Provides additional information
//     // #[arg(short, long, default_value_t = false)]
//     // verbose: bool,

//     // #[command(subcommand)]
//     // command: Groups,
// }

#[derive(Subcommand, Debug)]
enum Commands {
    /// Application Programming Interface.
    Api {},
    /// Applications available for installation.
    Apps {},
    /// Work with the device.
    Device {},
    /// Work with the emulator virtualbox.
    Emulator {},
    /// Work with Flutter for Aurora OS.
    Flutter {},
    /// Work with Platform Aurora SDK.
    Psdk {},
    /// Work with Aurora SDK.
    Sdk {},
    /// Additional application settings.
    Settings {},
    /// Работа с Visual Studio Code.
    Vscode {},

    // Stash(StashArgs),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
struct StashArgs {
    /// Provides additional information
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn main() {
    let _ = Cli::parse();



    // if m.is_present("help") {
    //     cli.print_help().unwrap();
    //     return;
    // }

    // for _ in 0..cli.count {
    //     println!("Hello {}!", cli.name);
    // }
}
