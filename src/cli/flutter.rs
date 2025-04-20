use clap::Args;

use crate::feature::flutter_available::incoming::FlutterAvailableIncoming;
use crate::feature::flutter_download::incoming::FlutterDownloadIncoming;
use crate::feature::flutter_info::incoming::FlutterInfoIncoming;
use crate::feature::flutter_install::incoming::FlutterInstallIncoming;
use crate::feature::flutter_terminal::incoming::FlutterTerminalIncoming;
use crate::feature::flutter_uninstall::incoming::FlutterUninstallIncoming;
use crate::feature::incoming::TraitIncoming;
use crate::feature::outgoing::OutgoingType;

#[derive(Args)]
#[command(arg_required_else_help = true)]
#[group(multiple = false)]
pub struct FlutterArgs {
    /// Информация по установленным Flutter SDK
    #[arg(long, default_value_t = false)]
    info: bool,
    /// Информация по доступным Flutter SDK
    #[arg(short, long, default_value_t = false)]
    available: bool,
    /// Открыть терминал с окружением Flutter
    #[arg(short, long, default_value_t = false)]
    terminal: bool,
    /// Скачать Flutter SDK
    #[arg(short, long, default_value_t = false)]
    download: bool,
    /// Установить Flutter SDK
    #[arg(short, long, default_value_t = false)]
    install: bool,
    /// Удалить Flutter SDK
    #[arg(short, long, default_value_t = false)]
    uninstall: bool,
    /// Показать это сообщение и выйти
    #[clap(short='h', long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,
}

pub fn run(arg: FlutterArgs) {
    if arg.available {
        FlutterAvailableIncoming::new().run(OutgoingType::Cli).print();
        return;
    }
    if arg.info {
        FlutterInfoIncoming::new().run(OutgoingType::Cli).print();
        return;
    }
    if arg.terminal {
        FlutterTerminalIncoming::new().run(OutgoingType::Cli).print();
        return;
    }
    if arg.download {
        FlutterDownloadIncoming::new().run(OutgoingType::Cli).print();
        return;
    }
    if arg.install {
        FlutterInstallIncoming::new().run(OutgoingType::Cli).print();
        return;
    }
    if arg.uninstall {
        FlutterUninstallIncoming::new().run(OutgoingType::Cli).print();
        return;
    }
}
