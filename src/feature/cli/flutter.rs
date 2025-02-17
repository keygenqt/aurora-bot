use clap::Args;

use crate::models::client::flutter_info::incoming::FlutterInfoIncoming;
use crate::models::client::flutter_terminal::incoming::FlutterTerminalIncoming;
use crate::models::client::incoming::TraitIncoming;
use crate::models::client::outgoing::OutgoingType;

#[derive(Args)]
#[command(arg_required_else_help = true)]
#[group(multiple = false)]
pub struct FlutterArgs {
    /// Информация по доступным Flutter SDK
    #[arg(short, long, default_value_t = false)]
    info: bool,

    /// Открыть терминал с окружением Flutter
    #[arg(short, long, default_value_t = false)]
    terminal: bool,

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
}
