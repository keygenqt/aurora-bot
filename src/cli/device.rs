use std::path::PathBuf;

use clap::Args;
use clap::Subcommand;

use crate::feature::demo_app_info::incoming::DemoAppInfoIncoming;
use crate::feature::demo_app_info::outgoing::DemoAppInfoOutgoing;
use crate::feature::device_info::incoming::DeviceInfoIncoming;
use crate::feature::device_package_install::incoming::DevicePackageInstallIncoming;
use crate::feature::device_package_run::incoming::DevicePackageRunIncoming;
use crate::feature::device_package_uninstall::incoming::DevicePackageUninstallIncoming;
use crate::feature::device_screenshot::incoming::DeviceScreenshotIncoming;
use crate::feature::device_terminal::incoming::DeviceTerminalIncoming;
use crate::feature::device_upload::incoming::DeviceUploadIncoming;
use crate::feature::incoming::DataIncoming;
use crate::feature::incoming::TraitIncoming;
use crate::feature::outgoing::OutgoingType;
use crate::tools::macros::print_error;
use crate::tools::utils;

#[derive(Args)]
#[command(arg_required_else_help = true)]
#[group(multiple = false)]
pub struct DeviceArgs {
    /// Subcommand
    #[command(subcommand)]
    command: Option<DeviceArgsGroup>,
    /// Информация по доступным устройствам
    #[arg(long, default_value_t = false)]
    info: bool,
    /// Сделать скриншот
    #[arg(short, long, default_value_t = false)]
    screenshot: bool,
    /// Открыть терминал
    #[arg(short, long, default_value_t = false)]
    terminal: bool,
    /// Скачать файл в каталог ~/Download
    #[arg(short, long, value_name = "url")]
    download: Option<String>,
    /// Загрузить файл в каталог ~/Download
    #[arg(short, long, value_name = "path")]
    upload: Option<PathBuf>,
    /// Показать это сообщение и выйти
    #[clap(short='h', long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,
}

#[derive(Subcommand)]
enum DeviceArgsGroup {
    /// Работа с пакетами
    #[command(short_flag = 'p')]
    Package(DevicePackageArgs),
}

#[derive(Args)]
#[group(multiple = false)]
#[command(arg_required_else_help = true)]
pub struct DevicePackageArgs {
    /// Установить пакет
    #[arg(short, long, value_name = "path")]
    install: Option<PathBuf>,
    /// Скачать и установить пакет
    #[arg(long, value_name = "url")]
    install_url: Option<String>,
    /// Установить демо приложение
    #[arg(long, default_value_t = false)]
    install_demo: bool,

    /// Удалить пакет c автоматическим поиском
    #[arg(short, long, default_value_t = false)]
    uninstall: bool,
    /// Удалить пакет по package-name
    #[arg(long, value_name = "package")]
    uninstall_name: Option<String>,

    /// Запустить пакет c автоматическим поиском
    #[arg(short, long, default_value_t = false)]
    run: bool,
    /// Запустить пакет по package-name
    #[arg(long, value_name = "package")]
    run_name: Option<String>,

    /// Показать это сообщение и выйти
    #[clap(short='h', long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,
}

pub fn run(arg: DeviceArgs) {
    // Options
    if arg.info {
        DeviceInfoIncoming::new().run(OutgoingType::Cli).print();
        return;
    }
    if arg.screenshot {
        DeviceScreenshotIncoming::new().run(OutgoingType::Cli).print();
        return;
    }
    if arg.terminal {
        DeviceTerminalIncoming::new().run(OutgoingType::Cli).print();
        return;
    }
    if let Some(url) = arg.download {
        match utils::get_https_url(url) {
            Some(url) => DeviceUploadIncoming::new_url(url).run(OutgoingType::Cli).print(),
            None => print_error!("проверьте url файла"),
        }
        return;
    }
    if let Some(path) = arg.upload {
        match utils::path_to_absolute(&path) {
            Some(path) => DeviceUploadIncoming::new_path(path).run(OutgoingType::Cli).print(),
            None => print_error!("проверьте путь к файлу"),
        }
        return;
    }
    // Commands
    if let Some(command) = arg.command {
        match command {
            DeviceArgsGroup::Package(arg) => {
                if let Some(path) = arg.install {
                    match utils::path_to_absolute(&path) {
                        Some(path) => DevicePackageInstallIncoming::new_path(path)
                            .run(OutgoingType::Cli)
                            .print(),
                        None => print_error!("проверьте путь к файлу"),
                    }
                    return;
                }
                if let Some(url) = arg.install_url {
                    match utils::get_https_url(url) {
                        Some(url) => DevicePackageInstallIncoming::new_urls(vec![url])
                            .run(OutgoingType::Cli)
                            .print(),
                        None => print_error!("проверьте url файла"),
                    }
                    return;
                }
                if arg.install_demo {
                    // Relations features via outgoing
                    let result = DataIncoming::get_model(&DemoAppInfoIncoming::new().run(OutgoingType::Cli).to_json());
                    if let Ok(json) = result {
                        match serde_json::from_str::<DemoAppInfoOutgoing>(&json) {
                            Ok(outgoing) => DevicePackageInstallIncoming::new_urls(vec![
                                outgoing.model.url_aarch64,
                                outgoing.model.url_armv7hl
                            ])
                                .run(OutgoingType::Cli)
                                .print(),
                            Err(_) => print_error!("ошибка получения данных"),
                        }
                    } else {
                        print_error!("ошибка получения данных");
                    }
                    return;
                }
                if let Some(package) = arg.uninstall_name {
                    DevicePackageUninstallIncoming::new_package(package)
                        .run(OutgoingType::Cli)
                        .print();
                    return;
                }
                if arg.uninstall {
                    DevicePackageUninstallIncoming::new().run(OutgoingType::Cli).print();
                    return;
                }
                if let Some(package) = arg.run_name {
                    DevicePackageRunIncoming::new_package(package)
                        .run(OutgoingType::Cli)
                        .print();
                    return;
                }
                if arg.run {
                    DevicePackageRunIncoming::new().run(OutgoingType::Cli).print();
                    return;
                }
            }
        }
    }
}
