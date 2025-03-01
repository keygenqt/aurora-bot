use std::path::PathBuf;

use clap::Args;
use clap::Subcommand;

use crate::models::client::emulator_close::incoming::EmulatorCloseIncoming;
use crate::models::client::emulator_info::incoming::EmulatorInfoIncoming;
use crate::models::client::emulator_open::incoming::EmulatorOpenIncoming;
use crate::models::client::emulator_record_start::incoming::EmulatorRecordStartIncoming;
use crate::models::client::emulator_record_stop::incoming::EmulatorRecordStopIncoming;
use crate::models::client::emulator_record_stop::incoming::EmulatorRecordStopType;
use crate::models::client::emulator_screenshot::incoming::EmulatorScreenshotIncoming;
use crate::models::client::emulator_terminal::incoming::EmulatorTerminalIncoming;
use crate::models::client::emulator_upload::incoming::EmulatorUploadIncoming;
use crate::models::client::incoming::TraitIncoming;
use crate::models::client::outgoing::OutgoingType;
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
    #[arg(short, long, default_value_t = false)]
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
            EmulatorArgsGroup::Terminal(arg) => {
                EmulatorTerminalIncoming::new(arg.root).run(OutgoingType::Cli).print();
            }
        }
    }
}
