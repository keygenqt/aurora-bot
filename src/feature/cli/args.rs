use clap::{Args, Subcommand};

use crate::models::incoming::device_info::DeviceInfoIncoming;
use crate::models::incoming::emulator_close::EmulatorCloseIncoming;
use crate::models::incoming::emulator_info::EmulatorInfoIncoming;
use crate::models::incoming::flutter_info::FlutterInfoIncoming;
use crate::models::incoming::psdk_info::PsdkInfoIncoming;
use crate::models::incoming::sdk_info::SdkInfoIncoming;
use crate::models::{
    incoming::{emulator_start::EmulatorStartIncoming, Incoming},
    outgoing::OutgoingType,
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
    /// Работа с устройством
    Device(DeviceArgs),
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
    start: bool,

    /// Закрыть эмулятор
    #[arg(short, long, default_value_t = false)]
    close: bool,
}

#[derive(Args)]
#[command(arg_required_else_help = true)]
#[group(multiple = false)]
pub struct FlutterArgs {
    /// Информация по доступным Flutter SDK
    #[arg(short, long, default_value_t = false)]
    info: bool,
}

#[derive(Args)]
#[command(arg_required_else_help = true)]
#[group(multiple = false)]
pub struct PsdkArgs {
    /// Информация по доступным Platform SDK
    #[arg(short, long, default_value_t = false)]
    info: bool,
}

#[derive(Args)]
#[command(arg_required_else_help = true)]
#[group(multiple = false)]
pub struct SdkArgs {
    /// Информация по доступным Аврора SDK
    #[arg(short, long, default_value_t = false)]
    info: bool,
}

/// Handling interface events
pub async fn run(arg: CliArgs) {
    match arg.command.unwrap() {
        CliCommands::Device(arg) => {
            if arg.info {
                Incoming::handler(DeviceInfoIncoming::new(), OutgoingType::Cli)
                    .await
                    .print()
            }
        }
        CliCommands::Emulator(arg) => {
            if arg.info {
                Incoming::handler(EmulatorInfoIncoming::new(), OutgoingType::Cli)
                    .await
                    .print()
            }
            if arg.start {
                Incoming::handler(EmulatorStartIncoming::new(), OutgoingType::Cli)
                    .await
                    .print()
            }
            if arg.close {
                Incoming::handler(EmulatorCloseIncoming::new(), OutgoingType::Cli)
                    .await
                    .print()
            }
        }
        CliCommands::Flutter(arg) => {
            if arg.info {
                Incoming::handler(FlutterInfoIncoming::new(), OutgoingType::Cli)
                    .await
                    .print()
            }
        }
        CliCommands::Psdk(arg) => {
            if arg.info {
                Incoming::handler(PsdkInfoIncoming::new(), OutgoingType::Cli)
                    .await
                    .print()
            }
        }
        CliCommands::Sdk(arg) => {
            if arg.info {
                Incoming::handler(SdkInfoIncoming::new(), OutgoingType::Cli)
                    .await
                    .print()
            }
        }
    }
}
