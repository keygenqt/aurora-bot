use clap::Args;

use crate::feature::incoming::TraitIncoming;
use crate::feature::outgoing::OutgoingType;
use crate::feature::sdk_available::incoming::SdkAvailableIncoming;
use crate::feature::sdk_download::incoming::SdkDownloadIncoming;
use crate::feature::sdk_ide_close::incoming::SdkIdeCloseIncoming;
use crate::feature::sdk_ide_open::incoming::SdkIdeOpenIncoming;
use crate::feature::sdk_info::incoming::SdkInfoIncoming;
use crate::feature::sdk_tools::incoming::SdkToolsIncoming;

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
    /// Открыть IDE
    #[arg(short, long, default_value_t = false)]
    open: bool,
    /// Закрыть IDE
    #[arg(short, long, default_value_t = false)]
    close: bool,
    /// Скачать Аврора SDK
    #[arg(short, long, default_value_t = false)]
    download: bool,
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
    if arg.open {
        SdkIdeOpenIncoming::new().run(OutgoingType::Cli).print();
        return;
    }
    if arg.close {
        SdkIdeCloseIncoming::new().run(OutgoingType::Cli).print();
        return;
    }
    if arg.download {
        SdkDownloadIncoming::new().run(OutgoingType::Cli).print();
        return;
    }
    if arg.tools {
        SdkToolsIncoming::new().run(OutgoingType::Cli).print();
        return;
    }
}
