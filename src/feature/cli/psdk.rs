use clap::Args;

use crate::models::client::incoming::TraitIncoming;
use crate::models::client::outgoing::OutgoingType;
use crate::models::client::psdk_available::incoming::PsdkAvailableIncoming;
use crate::models::client::psdk_info::incoming::PsdkInfoIncoming;
use crate::models::client::psdk_terminal::incoming::PsdkTerminalIncoming;

#[derive(Args)]
#[command(arg_required_else_help = true)]
#[group(multiple = false)]
pub struct PsdkArgs {
    /// Информация по доступным Platform SDK
    #[arg(short, long, default_value_t = false)]
    available: bool,
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

pub fn run(arg: PsdkArgs) {
    if arg.available {
        PsdkAvailableIncoming::new().run(OutgoingType::Cli).print();
        return;
    }
    if arg.info {
        PsdkInfoIncoming::new().run(OutgoingType::Cli).print();
        return;
    }
    if arg.terminal {
        PsdkTerminalIncoming::new().run(OutgoingType::Cli).print();
        return;
    }
}
