use clap::{Args, Subcommand};

use crate::models::client::flutter_available::incoming::FlutterAvailableIncoming;
use crate::models::client::flutter_info::incoming::FlutterInfoIncoming;
use crate::models::client::flutter_terminal::incoming::FlutterTerminalIncoming;
use crate::models::client::incoming::TraitIncoming;
use crate::models::client::outgoing::OutgoingType;

#[derive(Args)]
#[command(arg_required_else_help = true)]
#[group(multiple = false)]
pub struct FlutterArgs {
    /// Subcommand
    #[command(subcommand)]
    command: Option<FlutterArgsGroup>,
    /// Информация по установленным Flutter SDK
    #[arg(short, long, default_value_t = false)]
    info: bool,
    /// Открыть терминал с окружением Flutter
    #[arg(short, long, default_value_t = false)]
    terminal: bool,
    /// Показать это сообщение и выйти
    #[clap(short='h', long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,
}

#[derive(Subcommand)]
enum FlutterArgsGroup {
    /// Информация по доступным Flutter SDK
    #[command(short_flag = 'a')]
    Available(FlutterAvailableArgs),
}

#[derive(Args)]
#[group(multiple = false)]
pub struct FlutterAvailableArgs {
    /// Вывести все найденные версии
    #[arg(short, long, default_value_t = false)]
    all: bool,
    /// Показать это сообщение и выйти
    #[clap(short='h', long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,
}

pub fn run(arg: FlutterArgs) {
    if arg.info {
        FlutterInfoIncoming::new().run(OutgoingType::Cli).print();
    }
    if arg.terminal {
        FlutterTerminalIncoming::new().run(OutgoingType::Cli).print();
    }
    // Commands
    if let Some(command) = arg.command {
        match command {
            FlutterArgsGroup::Available(arg) => {
                FlutterAvailableIncoming::new(arg.all).run(OutgoingType::Cli).print();
            }
        }
    }
}
