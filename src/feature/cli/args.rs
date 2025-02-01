use clap::{Args, Subcommand};

use crate::{
    app::api::{enums::SendType, handler::handler_incoming, incoming::Incoming},
    utils::{macros::print_error, methods::print_outgoing},
};

/// Классическая командная строка
#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct CliArgs {
    #[command(subcommand)]
    command: Option<CliCommands>,
}

#[derive(Subcommand, Debug)]
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

#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct AppsArgs {
    /// Получите список доступных приложений для ОС Аврора
    #[arg(short, long, default_value_t = false)]
    available: bool,

    /// Установите приложение на устройство или эмулятор
    #[arg(short, long, default_value_t = false)]
    install: bool,
}

#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct EmulatorArgs {
    /// Запустить эмулятор
    #[arg(short, long, default_value_t = false)]
    start: bool,
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
                let incoming = Incoming::emulator_start();
                match handler_incoming(&incoming, SendType::Cli).await {
                    Ok(outgoing) => print_outgoing(&outgoing),
                    Err(error) => print_error!(error),
                }
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
