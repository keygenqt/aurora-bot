use std::path::PathBuf;

use clap::Args;
use clap::Subcommand;

use crate::feature::demo_app_info::incoming::DemoAppInfoIncoming;
use crate::feature::demo_app_info::outgoing::DemoAppInfoOutgoing;
use crate::feature::emulator_close::incoming::EmulatorCloseIncoming;
use crate::feature::emulator_info::incoming::EmulatorInfoIncoming;
use crate::feature::emulator_open::incoming::EmulatorOpenIncoming;
use crate::feature::emulator_package_install::incoming::EmulatorPackageInstallIncoming;
use crate::feature::emulator_package_run::incoming::EmulatorPackageRunIncoming;
use crate::feature::emulator_package_uninstall::incoming::EmulatorPackageUninstallIncoming;
use crate::feature::emulator_record_start::incoming::EmulatorRecordStartIncoming;
use crate::feature::emulator_record_stop::incoming::EmulatorRecordStopIncoming;
use crate::feature::emulator_record_stop::incoming::EmulatorRecordStopType;
use crate::feature::emulator_screenshot::incoming::EmulatorScreenshotIncoming;
use crate::feature::emulator_terminal::incoming::EmulatorTerminalIncoming;
use crate::feature::emulator_upload::incoming::EmulatorUploadIncoming;
use crate::feature::incoming::DataIncoming;
use crate::feature::incoming::TraitIncoming;
use crate::feature::outgoing::OutgoingType;
use crate::tools::macros::print_error;
use crate::tools::utils;

#[derive(Args)]
#[command(arg_required_else_help = true)]
#[group(multiple = false)]
pub struct EmulatorArgs {
    /// Subcommand
    #[command(subcommand)]
    command: Option<EmulatorArgsGroup>,
    /// Информация по доступным эмуляторам
    #[arg(long, default_value_t = false)]
    info: bool,
    /// Открыть эмулятор
    #[arg(short, long, default_value_t = false)]
    open: bool,
    /// Закрыть эмулятор
    #[arg(short, long, default_value_t = false)]
    close: bool,
    /// Сделать скриншот
    #[arg(short, long, default_value_t = false)]
    screenshot: bool,
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
enum EmulatorArgsGroup {
    /// Запустить эмулятор headless с VNC
    #[command(short_flag = 'v')]
    Vnc(EmulatorVncOpenArgs),
    /// Запись видео
    #[command(short_flag = 'r')]
    Record(EmulatorRecordArgs),
    /// Работа с пакетами
    #[command(short_flag = 'p')]
    Package(EmulatorPackageArgs),
    /// Открыть терминал
    #[command(short_flag = 't')]
    Terminal(EmulatorTerminalArgs),
}

#[derive(Args)]
pub struct EmulatorVncOpenArgs {
    /// Пароль доступа к VNC
    #[arg(short='w', long, default_value_t = String::from("0000"))]
    password: String,
    /// Порт доступа к VNC
    #[arg(short, long, default_value_t = 3389)]
    port: u64,
    /// Показать это сообщение и выйти
    #[clap(short='h', long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,
}

#[derive(Args)]
#[group(multiple = false)]
pub struct EmulatorRecordArgs {
    /// Остановить запись без конвертации
    #[arg(short, long, default_value_t = false)]
    raw_stop: bool,
    /// Остановить запись и создать Mp4
    #[arg(short, long, default_value_t = false)]
    mp4_stop: bool,
    /// Остановить запись и создать Gif
    #[arg(short, long, default_value_t = false)]
    gif_stop: bool,
    /// Показать это сообщение и выйти
    #[clap(short='h', long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,
}

#[derive(Args)]
#[group(multiple = false)]
#[command(arg_required_else_help = true)]
pub struct EmulatorPackageArgs {
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

#[derive(Args)]
#[group(multiple = false)]
pub struct EmulatorTerminalArgs {
    /// Открыть от root пользователя
    #[arg(short, long, default_value_t = false)]
    root: bool,
    /// Показать это сообщение и выйти
    #[clap(short='h', long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,
}

pub fn run(arg: EmulatorArgs) {
    // Options
    if arg.info {
        EmulatorInfoIncoming::new().run(OutgoingType::Cli).print();
        return;
    }
    if arg.open {
        EmulatorOpenIncoming::new().run(OutgoingType::Cli).print();
        return;
    }
    if arg.close {
        EmulatorCloseIncoming::new().run(OutgoingType::Cli).print();
        return;
    }
    if arg.screenshot {
        EmulatorScreenshotIncoming::new().run(OutgoingType::Cli).print();
        return;
    }
    if let Some(url) = arg.download {
        match utils::get_https_url(url) {
            Some(url) => EmulatorUploadIncoming::new_url(url).run(OutgoingType::Cli).print(),
            None => print_error!("проверьте url файла"),
        }
        return;
    }
    if let Some(path) = arg.upload {
        match utils::path_to_absolute(&path) {
            Some(path) => EmulatorUploadIncoming::new_path(path).run(OutgoingType::Cli).print(),
            None => print_error!("проверьте путь к файлу"),
        }
        return;
    }
    // Commands
    if let Some(command) = arg.command {
        match command {
            EmulatorArgsGroup::Vnc(arg) => {
                EmulatorOpenIncoming::new_vnc(arg.password, arg.port)
                    .run(OutgoingType::Cli)
                    .print();
            }
            EmulatorArgsGroup::Record(arg) => {
                if arg.raw_stop {
                    EmulatorRecordStopIncoming::new(EmulatorRecordStopType::Raw)
                        .run(OutgoingType::Cli)
                        .print();
                    return;
                }
                if arg.mp4_stop {
                    EmulatorRecordStopIncoming::new(EmulatorRecordStopType::Mp4)
                        .run(OutgoingType::Cli)
                        .print();
                    return;
                }
                if arg.gif_stop {
                    EmulatorRecordStopIncoming::new(EmulatorRecordStopType::Gif)
                        .run(OutgoingType::Cli)
                        .print();
                    return;
                }
                EmulatorRecordStartIncoming::new().run(OutgoingType::Cli).print();
            }
            EmulatorArgsGroup::Package(arg) => {
                if let Some(path) = arg.install {
                    match utils::path_to_absolute(&path) {
                        Some(path) => EmulatorPackageInstallIncoming::new_path(path)
                            .run(OutgoingType::Cli)
                            .print(),
                        None => print_error!("проверьте путь к файлу"),
                    }
                    return;
                }
                if let Some(url) = arg.install_url {
                    match utils::get_https_url(url) {
                        Some(url) => EmulatorPackageInstallIncoming::new_url(url)
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
                            Ok(outgoing) => EmulatorPackageInstallIncoming::new_url(outgoing.model.url_x86_64)
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
                    EmulatorPackageUninstallIncoming::new_package(package)
                        .run(OutgoingType::Cli)
                        .print();
                    return;
                }
                if arg.uninstall {
                    EmulatorPackageUninstallIncoming::new().run(OutgoingType::Cli).print();
                    return;
                }
                if let Some(package) = arg.run_name {
                    EmulatorPackageRunIncoming::new_package(true, package)
                        .run(OutgoingType::Cli)
                        .print();
                    return;
                }
                if arg.run {
                    EmulatorPackageRunIncoming::new(true).run(OutgoingType::Cli).print();
                    return;
                }
            }
            EmulatorArgsGroup::Terminal(arg) => {
                EmulatorTerminalIncoming::new(arg.root).run(OutgoingType::Cli).print();
            }
        }
    }
}
