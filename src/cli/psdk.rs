use std::path::PathBuf;

use clap::Args;
use clap::Subcommand;

use crate::feature::incoming::TraitIncoming;
use crate::feature::outgoing::OutgoingType;
use crate::feature::psdk_available::incoming::PsdkAvailableIncoming;
use crate::feature::psdk_download::incoming::PsdkDownloadIncoming;
use crate::feature::psdk_info::incoming::PsdkInfoIncoming;
use crate::feature::psdk_install::incoming::PsdkInstallIncoming;
use crate::feature::psdk_package_sign::incoming::PsdkPackageSignIncoming;
use crate::feature::psdk_target_package_find::incoming::PsdkTargetPackageFindIncoming;
use crate::feature::psdk_target_package_install::incoming::PsdkTargetPackageInstallIncoming;
use crate::feature::psdk_target_package_uninstall::incoming::PsdkTargetPackageUninstallIncoming;
use crate::feature::psdk_terminal::incoming::PsdkTerminalIncoming;
use crate::feature::psdk_uninstall::incoming::PsdkUninstallIncoming;
use crate::tools::macros::print_error;
use crate::tools::utils;

#[derive(Args)]
#[command(arg_required_else_help = true)]
#[group(multiple = false)]
pub struct PsdkArgs {
    /// Subcommand
    #[command(subcommand)]
    command: Option<PsdkArgsGroup>,
    /// Информация по установленным Platform SDK
    #[arg(long, default_value_t = false)]
    info: bool,
    /// Информация по доступным Platform SDK
    #[arg(short, long, default_value_t = false)]
    available: bool,
    /// Открыть терминал с окружением Platform SDK
    #[arg(short, long, default_value_t = false)]
    terminal: bool,
    /// Скачать Platform SDK
    #[arg(short, long, default_value_t = false)]
    download: bool,
    /// Установить Platform SDK
    #[arg(short, long, default_value_t = false)]
    install: bool,
    /// Удалить Platform SDK
    #[arg(short, long, default_value_t = false)]
    uninstall: bool,
    /// Установка через терминал по ID
    #[arg(long, value_name = "id", hide = true)]
    terminal_install: Option<String>,
    /// Установка через терминал по ID
    #[arg(long, value_name = "id", hide = true)]
    terminal_uninstall: Option<String>,
    /// Показать это сообщение и выйти
    #[clap(short='h', long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,
}

#[derive(Subcommand)]
enum PsdkArgsGroup {
    /// Работа с пакетами
    #[command(short_flag = 'p')]
    Package(PsdkPackageArgs),
}

#[derive(Args)]
#[group(multiple = false)]
#[command(arg_required_else_help = true)]
pub struct PsdkPackageArgs {
    /// Подписать пакет открытым ключом
    #[arg(short, long, value_name = "path")]
    sign: Option<PathBuf>,
    /// Поиск среди локальных пакетов
    #[arg(short, long, value_name = "package")]
    find: Option<String>,
    /// Установить пакет
    #[arg(short, long, value_name = "path")]
    install: Option<PathBuf>,
    /// Удалить пакет по package-name
    #[arg(short, long, value_name = "package")]
    uninstall: Option<String>,
    /// Показать это сообщение и выйти
    #[clap(short='h', long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,
}

pub fn run(arg: PsdkArgs) {
    // Options
    if arg.info {
        PsdkInfoIncoming::new().run(OutgoingType::Cli).print();
        return;
    }
    if arg.available {
        PsdkAvailableIncoming::new().run(OutgoingType::Cli).print();
        return;
    }
    if arg.terminal {
        PsdkTerminalIncoming::new().run(OutgoingType::Cli).print();
        return;
    }
    if arg.download {
        PsdkDownloadIncoming::new().run(OutgoingType::Cli).print();
        return;
    }
    if arg.install {
        PsdkInstallIncoming::new().run(OutgoingType::Cli).print();
        return;
    }
    if arg.uninstall {
        PsdkUninstallIncoming::new().run(OutgoingType::Cli).print();
        return;
    }
    if let Some(id) = arg.terminal_install {
        // a1a27d604c6e8498a87b2f82ff1a3302
        PsdkInstallIncoming::new_id(id).run(OutgoingType::Cli).print();
        return;
    }
    if let Some(id) = arg.terminal_uninstall {
        // 3d883d22072d216f7224dbb7b076b74d
        PsdkUninstallIncoming::new_id(id).run(OutgoingType::Cli).print();
        return;
    }
    // Commands
    if let Some(command) = arg.command {
        match command {
            PsdkArgsGroup::Package(arg) => {
                if let Some(path) = arg.sign {
                    match utils::path_to_absolute(&path) {
                        Some(path) => {
                            PsdkPackageSignIncoming::new(path).run(OutgoingType::Cli).print();
                        }
                        None => print_error!("проверьте путь к файлу"),
                    }
                    return;
                }
                if let Some(package) = arg.find {
                    PsdkTargetPackageFindIncoming::new(package)
                        .run(OutgoingType::Cli)
                        .print();
                    return;
                }
                if let Some(path) = arg.install {
                    match utils::path_to_absolute(&path) {
                        Some(path) => {
                            PsdkTargetPackageInstallIncoming::new(path)
                                .run(OutgoingType::Cli)
                                .print();
                        }
                        None => print_error!("проверьте путь к файлу"),
                    }
                    return;
                }
                if let Some(package) = arg.uninstall {
                    PsdkTargetPackageUninstallIncoming::new(package)
                        .run(OutgoingType::Cli)
                        .print();
                    return;
                }
            }
        }
    }
}
