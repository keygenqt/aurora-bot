use clap::{Args, Subcommand};

use crate::models::device::model::DeviceModel;
use crate::models::emulator::model::EmulatorModel;
use crate::models::flutter::model::FlutterModel;
use crate::models::incoming::emulator_close::EmulatorCloseIncoming;
use crate::models::psdk::model::PsdkModel;
use crate::models::sdk::model::SdkModel;
use crate::models::{
    incoming::{emulator_start::EmulatorStartIncoming, Incoming},
    outgoing::OutgoingType,
};
use crate::utils::macros::print_error;

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
    /// Работа с Аврора Platform SDK
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
    /// Информация по доступным Аврора Platform SDK
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
                match DeviceModel::search().await {
                    Ok(models) => DeviceModel::print_list(models),
                    Err(_) => print_error!("не удалось получить данные"),
                };
            }
        }
        CliCommands::Emulator(arg) => {
            if arg.info {
                match EmulatorModel::search().await {
                    Ok(models) => EmulatorModel::print_list(models),
                    Err(_) => print_error!("не удалось получить данные"),
                };
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
                match FlutterModel::search().await {
                    Ok(models) => FlutterModel::print_list(models),
                    Err(_) => print_error!("не удалось получить данные"),
                };
            }
        }
        CliCommands::Psdk(arg) => {
            if arg.info {
                match PsdkModel::search().await {
                    Ok(models) => PsdkModel::print_list(models),
                    Err(_) => print_error!("не удалось получить данные"),
                };
            }
        }
        CliCommands::Sdk(arg) => {
            if arg.info {
                match SdkModel::search().await {
                    Ok(models) => SdkModel::print_list(models),
                    Err(_) => print_error!("не удалось получить данные"),
                };
            }
        }
    }
}
