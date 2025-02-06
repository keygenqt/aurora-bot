use clap::{Args, Subcommand};

use crate::models::incoming::emulator_close::IncomingEmulatorClose;
use crate::models::{
    incoming::{emulator_start::IncomingEmulatorStart, Incoming},
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
    // /// Приложения доступные для установки
    // Apps(AppsArgs),
    // /// Работа с устройством
    // Device{},
    /// Работа с эмулятором в virtualbox
    Emulator(EmulatorArgs),
    // /// Работа с Аврора SDK
    // Sdk {},
    // /// Работа с Аврора Platform SDK
    // Psdk {},
    // /// Работа с Flutter для ОС Aurora
    // Flutter {},
    // /// Работа с Visual Studio Code
    // Vscode {},
}

#[derive(Args)]
#[command(arg_required_else_help = true)]
pub struct AppsArgs {
    /// Получите список доступных приложений для ОС Аврора
    #[arg(short, long, default_value_t = false)]
    available: bool,

    /// Установите приложение на устройство или эмулятор
    #[arg(short, long, default_value_t = false)]
    install: bool,
}

#[derive(Args)]
#[command(arg_required_else_help = true)]
pub struct EmulatorArgs {
    /// Запустить эмулятор
    #[arg(short, long, default_value_t = false)]
    start: bool,
    /// Закрыть эмулятор
    #[arg(short, long, default_value_t = false)]
    close: bool,
}

/// Handling interface events
pub async fn run(arg: CliArgs) {
    match arg.command.unwrap() {
        // CliCommands::Apps(arg) => {
        //     if arg.available {
        //         println!("Get available")
        //     }
        //     if arg.install {
        //         println!("Install package")
        //     }
        // }
        // CliCommands::Device {} => {
        //     println!("Device")
        // }
        CliCommands::Emulator(arg) => {
            if arg.start {
                Incoming::handler(IncomingEmulatorStart::new(), OutgoingType::Cli)
                    .await
                    .print()
            }
            if arg.close {
                Incoming::handler(IncomingEmulatorClose::new(), OutgoingType::Cli)
                    .await
                    .print()
            }
        } // CliCommands::Sdk {} => {
          //     println!("Sdk")
          // }
          // CliCommands::Psdk {} => {
          //     println!("Psdk")
          // }
          // CliCommands::Flutter {} => {
          //     println!("Flutter")
          // }
          // CliCommands::Vscode {} => {
          //     println!("Vscode")
          // }
    }
}
