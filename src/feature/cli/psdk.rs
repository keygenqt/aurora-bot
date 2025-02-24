use clap::Args;
use clap::Subcommand;

use crate::models::client::incoming::TraitIncoming;
use crate::models::client::outgoing::OutgoingType;
use crate::models::client::psdk_available::incoming::PsdkAvailableIncoming;
use crate::models::client::psdk_info::incoming::PsdkInfoIncoming;
use crate::models::client::psdk_terminal::incoming::PsdkTerminalIncoming;

#[derive(Args)]
#[command(arg_required_else_help = true)]
#[group(multiple = false)]
pub struct PsdkArgs {
    /// Subcommand
    #[command(subcommand)]
    command: Option<PsdkArgsGroup>,
    /// Информация по установленным Platform SDK
    #[arg(short, long, default_value_t = false)]
    info: bool,
    /// Открыть терминал с окружением Platform SDK
    #[arg(short, long, default_value_t = false)]
    terminal: bool,
    /// Показать это сообщение и выйти
    #[clap(short='h', long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,
}

#[derive(Subcommand)]
enum PsdkArgsGroup {
    /// Информация по доступным Platform SDK
    #[command(short_flag = 'a')]
    Available(PsdkAvailableArgs),
}

#[derive(Args)]
#[group(multiple = false)]
pub struct PsdkAvailableArgs {
    /// Вывести все найденные версии
    #[arg(short, long, default_value_t = false)]
    all: bool,
    /// Показать это сообщение и выйти
    #[clap(short='h', long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,
}

pub fn run(arg: PsdkArgs) {
    if arg.info {
        PsdkInfoIncoming::new().run(OutgoingType::Cli).print();
    }
    if arg.terminal {
        PsdkTerminalIncoming::new().run(OutgoingType::Cli).print();
    }
    // Commands
    if let Some(command) = arg.command {
        match command {
            PsdkArgsGroup::Available(arg) => {
                PsdkAvailableIncoming::new(arg.all).run(OutgoingType::Cli).print();
            }
        }
    }
}
