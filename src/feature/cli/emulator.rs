use clap::Args;
use clap::Subcommand;

use crate::models::client::emulator_close::incoming::EmulatorCloseIncoming;
use crate::models::client::emulator_info::incoming::EmulatorInfoIncoming;
use crate::models::client::emulator_open::incoming::EmulatorOpenIncoming;
use crate::models::client::emulator_open_vnc::incoming::EmulatorOpenVncIncoming;
use crate::models::client::emulator_record_disable::incoming::EmulatorRecordDisableIncoming;
use crate::models::client::emulator_record_enable::incoming::EmulatorRecordEnableIncoming;
use crate::models::client::emulator_screenshot::incoming::EmulatorScreenshotIncoming;
use crate::models::client::emulator_terminal::incoming::EmulatorTerminalIncoming;
use crate::models::client::incoming::TraitIncoming;
use crate::models::client::outgoing::OutgoingType;

#[derive(Args)]
#[command(arg_required_else_help = true)]
#[group(multiple = false)]
pub struct EmulatorArgs {
    /// Информация по доступным эмуляторам
    #[arg(short, long, default_value_t = false)]
    info: bool,

    /// Закрыть эмулятор
    #[arg(short, long, default_value_t = false)]
    close: bool,

    /// Сделать скриншот
    #[arg(short, long, default_value_t = false)]
    screenshot: bool,

    // Subcommand (multiple false ignore)
    #[command(subcommand)]
    command: Option<EmulatorArgsGroup>,

    /// Показать это сообщение и выйти
    #[clap(short='h', long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,
}

#[derive(Subcommand)]
enum EmulatorArgsGroup {
    /// Открыть эмулятор
    #[command(short_flag = 'o')]
    Open(EmulatorOpenArgs),

    /// Запись видео
    #[command(short_flag = 'r')]
    Record(EmulatorRecordArgs),

    /// Открыть терминал
    #[command(short_flag = 't')]
    Terminal(EmulatorTerminalArgs),
}

#[derive(Args)]
#[group(multiple = false)]
pub struct EmulatorOpenArgs {
    /// Запустить эмулятор headless с VNC
    #[arg(short, long, default_value_t = false)]
    vnc: bool,

    /// Показать это сообщение и выйти
    #[clap(short='h', long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,
}

#[derive(Args)]
#[group(multiple = false)]
pub struct EmulatorRecordArgs {
    /// Остановить запись
    #[arg(short, long, default_value_t = false)]
    stop: bool,

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
    if arg.close {
        EmulatorCloseIncoming::new().run(OutgoingType::Cli).print();
        return;
    }
    if arg.screenshot {
        EmulatorScreenshotIncoming::new().run(OutgoingType::Cli).print();
        return;
    }
    // Commands
    if let Some(command) = arg.command {
        match command {
            EmulatorArgsGroup::Open(arg) => {
                if arg.vnc {
                    EmulatorOpenVncIncoming::new().run(OutgoingType::Cli).print();
                    return;
                }
                EmulatorOpenIncoming::new().run(OutgoingType::Cli).print();
            }
            EmulatorArgsGroup::Record(arg) => {
                if arg.stop {
                    EmulatorRecordDisableIncoming::new().run(OutgoingType::Cli).print();
                    return;
                }
                EmulatorRecordEnableIncoming::new().run(OutgoingType::Cli).print();
            }
            EmulatorArgsGroup::Terminal(arg) => {
                EmulatorTerminalIncoming::new(arg.root).run(OutgoingType::Cli).print();
            }
        }
    }
}
