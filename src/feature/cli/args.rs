use clap::{Args, Subcommand};

/// Классическая командная строка
#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct CliArgs {
    #[command(subcommand)]
    command: Option<CliCommands>,
}

#[derive(Subcommand, Debug)]
pub enum CliCommands {
    /// Приложения доступные для установки
    Apps(AppsArgs),
    /// Работа с устройством
    Device {},
    /// Работа с эмулятором в virtualbox
    Emulator {},
    /// Работа с Аврора SDK
    Sdk {},
    /// Работа с Аврора Platform SDK
    Psdk {},
    /// Работа с Flutter для ОС Aurora
    Flutter {},
    /// Работа с Visual Studio Code
    Vscode {},
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

/// Handling interface events
#[allow(dead_code)]
pub async fn run(arg: CliArgs) {
    match arg.command.unwrap() {
        CliCommands::Apps(arg) => {
            if arg.available {
                println!("Get available")
            }
            if arg.install {
                println!("Install package")
            }
        }
        CliCommands::Device {} => {
            println!("Device")
        }
        CliCommands::Emulator {} => {
            println!("Emulator")
        }
        CliCommands::Sdk {} => {
            println!("Sdk")
        }
        CliCommands::Psdk {} => {
            println!("Psdk")
        }
        CliCommands::Flutter {} => {
            println!("Flutter")
        }
        CliCommands::Vscode {} => {
            println!("Vscode")
        }
    }
}
