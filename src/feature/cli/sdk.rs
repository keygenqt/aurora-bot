use clap::Args;
use clap::Subcommand;

use crate::models::client::incoming::TraitIncoming;
use crate::models::client::outgoing::OutgoingType;
use crate::models::client::sdk_available::incoming::SdkAvailableIncoming;
use crate::models::client::sdk_info::incoming::SdkInfoIncoming;
use crate::models::client::sdk_tools::incoming::SdkToolsIncoming;

#[derive(Args)]
#[command(arg_required_else_help = true)]
#[group(multiple = false)]
pub struct SdkArgs {
    /// Subcommand
    #[command(subcommand)]
    command: Option<SdkArgsGroup>,
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

#[derive(Subcommand)]
enum SdkArgsGroup {
    /// Информация по доступным Аврора SDK
    #[command(short_flag = 'a')]
    Available(SdkAvailableArgs),
}

#[derive(Args)]
#[group(multiple = false)]
pub struct SdkAvailableArgs {
    /// Вывести все найденные версии
    #[arg(short, long, default_value_t = false)]
    all: bool,
    /// Показать это сообщение и выйти
    #[clap(short='h', long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,
}

pub fn run(arg: SdkArgs) {
    if arg.info {
        SdkInfoIncoming::new().run(OutgoingType::Cli).print();
    }
    if arg.tools {
        SdkToolsIncoming::new().run(OutgoingType::Cli).print();
    }
    // Commands
    if let Some(command) = arg.command {
        match command {
            SdkArgsGroup::Available(arg) => {
                SdkAvailableIncoming::new(arg.all).run(OutgoingType::Cli).print();
            }
        }
    }
}
