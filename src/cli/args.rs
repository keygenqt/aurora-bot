use clap::{Args, Subcommand};
use color_eyre::owo_colors::OwoColorize;

use crate::utils;

/// Классическая командная строка
#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct CliArgs {
    #[command(subcommand)]
    command: Option<CliCommands>,
}

#[derive(Subcommand, Debug)]
pub enum CliCommands {
    /// Работа с Aurora Dataset
    Dataset(DatasetArgs),
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
pub struct DatasetArgs {
    /// Просто задай вопрос, найдем ответ
    #[arg(short, long, value_name="free text", num_args=1..=999)]
    search: Option<Vec<String>>,

    /// Получить отчет о состоянии Aurora Dataset
    #[arg(short, long, default_value_t = false)]
    info: bool,
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

/**
 * Run module command
 */
pub async fn run(arg: CliArgs) {
    match arg.command.unwrap() {
        CliCommands::Dataset(arg) => {
            if !arg.search.is_none() {
                let client = utils::requests::ClientRequest::new();
                match client
                    .auth(|link| println!("Перейдите по ссылки для авторизации: {}", link.blue()))
                    .await
                {
                    Ok(_) => {
                        match client
                            .get_search(arg.search.unwrap_or(vec![]).join(" "))
                            .await
                        {
                            Ok(value) => println!("{}", value),
                            Err(error) => println!("> Error: {}", error),
                        }
                    }
                    Err(error) => println!("> Error: {}", error),
                }
            }
        }
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
