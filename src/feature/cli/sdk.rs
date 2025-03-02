use clap::Args;

use crate::models::client::incoming::TraitIncoming;
use crate::models::client::outgoing::OutgoingType;
use crate::models::client::sdk_available::incoming::SdkAvailableIncoming;
use crate::models::client::sdk_info::incoming::SdkInfoIncoming;
use crate::models::client::sdk_tools::incoming::SdkToolsIncoming;

#[derive(Args)]
#[command(arg_required_else_help = true)]
#[group(multiple = false)]
pub struct SdkArgs {
    /// Информация по доступным Аврора SDK
    #[arg(short, long, default_value_t = false)]
    available: bool,
    /// Информация по установленным Аврора SDK
    #[arg(short, long, default_value_t = false)]
    info: bool,
    /// Открыть maintenance tools
    #[arg(short, long, default_value_t = false)]
    tools: bool,
    /// Показать это сообщение и выйти
    #[clap(short='h', long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,
}

pub fn run(arg: SdkArgs) {
    if arg.available {
        SdkAvailableIncoming::new().run(OutgoingType::Cli).print();
        return;
    }
    if arg.info {
        SdkInfoIncoming::new().run(OutgoingType::Cli).print();
        return;
    }
    if arg.tools {
        SdkToolsIncoming::new().run(OutgoingType::Cli).print();
        return;
    }
}
