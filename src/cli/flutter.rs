use std::path::PathBuf;

use clap::Args;

use crate::feature::flutter_available::incoming::FlutterAvailableIncoming;
use crate::feature::flutter_download::incoming::FlutterDownloadIncoming;
use crate::feature::flutter_info::incoming::FlutterInfoIncoming;
use crate::feature::flutter_install::incoming::FlutterInstallIncoming;
use crate::feature::flutter_project_format::incoming::FlutterProjectFormatIncoming;
use crate::feature::flutter_project_report::incoming::FlutterProjectReportIncoming;
use crate::feature::flutter_terminal::incoming::FlutterTerminalIncoming;
use crate::feature::flutter_uninstall::incoming::FlutterUninstallIncoming;
use crate::feature::incoming::TraitIncoming;
use crate::feature::outgoing::OutgoingType;
use crate::tools::macros::print_error;
use crate::tools::utils;

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
    /// Форматировать проект Dart/C++
    #[arg(short, long, value_name = "path")]
    format: Option<PathBuf>,
    /// Сформировать отчет по плагинам проекта Flutter
    #[arg(short, long, value_name = "path")]
    report: Option<PathBuf>,
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
    if let Some(path) = arg.format {
        match utils::path_to_absolute(&path) {
            Some(path) => {
                if path.is_dir() {
                    FlutterProjectFormatIncoming::new(path).run(OutgoingType::Cli).print();
                } else {
                    print_error!("укажите директорию проекта")
                }
            }
            None => print_error!("проверьте путь к проекту"),
        }
        return;
    }
    if let Some(path) = arg.report {
        match utils::path_to_absolute(&path) {
            Some(path) => {
                FlutterProjectReportIncoming::new_path(path)
                    .run(OutgoingType::Cli)
                    .print();
            }
            None => print_error!("проверьте путь к pubspec.yaml"),
        }
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
