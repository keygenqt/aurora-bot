use std::path::PathBuf;

use clap::Args;

use crate::feature::incoming::TraitIncoming;
use crate::feature::outgoing::OutgoingType;
use crate::feature::sdk_available::incoming::SdkAvailableIncoming;
use crate::feature::sdk_download::incoming::SdkDownloadIncoming;
use crate::feature::sdk_ide_close::incoming::SdkIdeCloseIncoming;
use crate::feature::sdk_ide_open::incoming::SdkIdeOpenIncoming;
use crate::feature::sdk_info::incoming::SdkInfoIncoming;
use crate::feature::sdk_install::incoming::SdkInstallIncoming;
use crate::feature::sdk_project_format::incoming::SdkProjectFormatIncoming;
use crate::feature::sdk_terminal::incoming::SdkTerminalIncoming;
use crate::feature::sdk_tools::incoming::SdkToolsIncoming;
use crate::feature::sdk_uninstall::incoming::SdkUninstallIncoming;
use crate::tools::macros::print_error;
use crate::tools::utils;

#[derive(Args)]
#[command(arg_required_else_help = true)]
#[group(multiple = false)]
pub struct SdkArgs {
    /// Информация по установленным Аврора SDK
    #[arg(long, default_value_t = false)]
    info: bool,
    /// Информация по доступным Аврора SDK
    #[arg(short, long, default_value_t = false)]
    available: bool,
    /// Открыть IDE
    #[arg(short, long, default_value_t = false)]
    open: bool,
    /// Закрыть IDE
    #[arg(short, long, default_value_t = false)]
    close: bool,
    /// Открыть терминал MB2
    #[arg(short, long, default_value_t = false)]
    terminal: bool,
    /// Открыть maintenance tools
    #[arg(short, long, default_value_t = false)]
    maintenance: bool,
    /// Форматировать проект Qt/C++
    #[arg(short, long, value_name = "path")]
    format: Option<PathBuf>,
    /// Скачать Аврора SDK
    #[arg(short, long, default_value_t = false)]
    download: bool,
    /// Установить Аврора SDK
    #[arg(short, long, default_value_t = false)]
    install: bool,
    /// Удалить Аврора SDK
    #[arg(short, long, default_value_t = false)]
    uninstall: bool,
    /// Показать это сообщение и выйти
    #[clap(short='h', long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,
}

pub fn run(arg: SdkArgs) {
    if arg.info {
        SdkInfoIncoming::new().run(OutgoingType::Cli).print();
        return;
    }
    if arg.available {
        SdkAvailableIncoming::new().run(OutgoingType::Cli).print();
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
    if arg.terminal {
        SdkTerminalIncoming::new().run(OutgoingType::Cli).print();
        return;
    }
    if arg.maintenance {
        SdkToolsIncoming::new().run(OutgoingType::Cli).print();
        return;
    }
    if let Some(path) = arg.format {
        match utils::path_to_absolute(&path) {
            Some(path) => {
                if path.is_dir() {
                    SdkProjectFormatIncoming::new(path).run(OutgoingType::Cli).print();
                } else {
                    print_error!("укажите директорию проекта")
                }
            }
            None => print_error!("проверьте путь к проекту"),
        }
        return;
    }
    if arg.download {
        SdkDownloadIncoming::new().run(OutgoingType::Cli).print();
        return;
    }
    if arg.install {
        SdkInstallIncoming::new().run(OutgoingType::Cli).print();
        return;
    }
    if arg.uninstall {
        SdkUninstallIncoming::new().run(OutgoingType::Cli).print();
        return;
    }
}
