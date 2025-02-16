use clap::{Args, Subcommand};

use crate::models::client::{
    emulator_close::incoming::EmulatorCloseIncoming, emulator_info::incoming::EmulatorInfoIncoming, emulator_open::incoming::EmulatorOpenIncoming, emulator_open_vnc::incoming::EmulatorOpenVncIncoming, emulator_record_disable::incoming::EmulatorRecordDisableIncoming, emulator_record_enable::incoming::EmulatorRecordEnableIncoming, emulator_screenshot::incoming::EmulatorScreenshotIncoming, emulator_terminal::incoming::EmulatorTerminalIncoming, emulator_terminal_root::incoming::EmulatorTerminalRootIncoming, flutter_info::incoming::FlutterInfoIncoming, flutter_terminal::incoming::FlutterTerminalIncoming, incoming::TraitIncoming, outgoing::OutgoingType, psdk_info::incoming::PsdkInfoIncoming, psdk_terminal::incoming::PsdkTerminalIncoming, sdk_info::incoming::SdkInfoIncoming, sdk_tools::incoming::SdkToolsIncoming
};

/// Классическая командная строка
#[derive(Args)]
#[command(arg_required_else_help = true)]
pub struct CliArgs {
    #[command(subcommand)]
    command: Option<CliCommands>,
}

#[derive(Subcommand)]
pub enum CliCommands {
    // /// Работа с устройством
    // Device(DeviceArgs),
    /// Работа с эмуляторами
    Emulator(EmulatorArgs),
    /// Работа с Flutter SDK
    Flutter(FlutterArgs),
    /// Работа с Platform SDK
    Psdk(PsdkArgs),
    /// Работа с Аврора SDK
    Sdk(SdkArgs),
}

#[derive(Args)]
#[command(arg_required_else_help = true)]
#[group(multiple = false)]
pub struct DeviceArgs {
    /// Информация по доступным устройствам
    #[arg(short, long, default_value_t = false)]
    info: bool,

    /// Открыть терминал с соединением ssh
    #[arg(short, long, default_value_t = false)]
    terminal: bool,
}

#[derive(Args)]
#[command(arg_required_else_help = true)]
#[group(multiple = false)]
pub struct EmulatorArgs {
    /// Информация по доступным эмуляторам
    #[arg(short, long, default_value_t = false)]
    info: bool,

    /// Запустить эмулятор
    #[arg(short, long, default_value_t = false)]
    open: bool,

    /// Запустить эмулятор headless с VNC
    #[arg(short, long, default_value_t = false)]
    vnc_open: bool,

    /// Закрыть эмулятор
    #[arg(short, long, default_value_t = false)]
    close: bool,

    /// Сделать скриншот
    #[arg(short, long, default_value_t = false)]
    screenshot: bool,

    /// Активировать запись видео
    #[arg(short, long, default_value_t = false)]
    enable_record: bool,

    /// Остановить запись видео и получить результат
    #[arg(short, long, default_value_t = false)]
    disable_record: bool,

    /// Открыть терминал с соединением ssh
    #[arg(short, long, default_value_t = false)]
    user_terminal: bool,

    /// Открыть терминал с соединением ssh пользователя root
    #[arg(short, long, default_value_t = false)]
    root_terminal: bool,
}

#[derive(Args)]
#[command(arg_required_else_help = true)]
#[group(multiple = false)]
pub struct FlutterArgs {
    /// Информация по доступным Flutter SDK
    #[arg(short, long, default_value_t = false)]
    info: bool,

    /// Открыть терминал с окружением Flutter
    #[arg(short, long, default_value_t = false)]
    terminal: bool,
}

#[derive(Args)]
#[command(arg_required_else_help = true)]
#[group(multiple = false)]
pub struct PsdkArgs {
    /// Информация по доступным Platform SDK
    #[arg(short, long, default_value_t = false)]
    info: bool,

    /// Открыть терминал с окружением Platform SDK
    #[arg(short, long, default_value_t = false)]
    terminal: bool,
}

#[derive(Args)]
#[command(arg_required_else_help = true)]
#[group(multiple = false)]
pub struct SdkArgs {
    /// Информация по доступным Аврора SDK
    #[arg(short, long, default_value_t = false)]
    info: bool,

    /// Открыть maintenance tools
    #[arg(short, long, default_value_t = false)]
    tools: bool,
}

/// Handling interface events
pub fn run(arg: CliArgs) {
    match arg.command.unwrap() {
        // CliCommands::Device(arg) => {
        //     if arg.info {
        //         // @todo
        //     }
        //     if arg.terminal {
        //         // @todo
        //     }
        // }
        CliCommands::Emulator(arg) => {
            if arg.info {
                EmulatorInfoIncoming::new().run(OutgoingType::Cli).print();
            }
            if arg.open {
                EmulatorOpenIncoming::new().run(OutgoingType::Cli).print();
            }
            if arg.vnc_open {
                EmulatorOpenVncIncoming::new().run(OutgoingType::Cli).print();
            }
            if arg.close {
                EmulatorCloseIncoming::new().run(OutgoingType::Cli).print();
            }
            if arg.screenshot {
                EmulatorScreenshotIncoming::new().run(OutgoingType::Cli).print();
            }
            if arg.enable_record {
                EmulatorRecordEnableIncoming::new().run(OutgoingType::Cli).print();
            }
            if arg.disable_record {
                EmulatorRecordDisableIncoming::new().run(OutgoingType::Cli).print();
            }
            if arg.user_terminal {
                EmulatorTerminalIncoming::new()
                    .run(OutgoingType::Cli)
                    .print();
            }
            if arg.root_terminal {
                EmulatorTerminalRootIncoming::new()
                    .run(OutgoingType::Cli)
                    .print();
            }
        }
        CliCommands::Flutter(arg) => {
            if arg.info {
                FlutterInfoIncoming::new().run(OutgoingType::Cli).print();
            }
            if arg.terminal {
                FlutterTerminalIncoming::new()
                    .run(OutgoingType::Cli)
                    .print();
            }
        }
        CliCommands::Psdk(arg) => {
            if arg.info {
                PsdkInfoIncoming::new().run(OutgoingType::Cli).print();
            }
            if arg.terminal {
                PsdkTerminalIncoming::new().run(OutgoingType::Cli).print();
            }
        }
        CliCommands::Sdk(arg) => {
            if arg.info {
                SdkInfoIncoming::new().run(OutgoingType::Cli).print();
            }
            if arg.tools {
                SdkToolsIncoming::new().run(OutgoingType::Cli).print();
            }
        }
    }
}
