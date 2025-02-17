use clap::Args;
use clap::Subcommand;

use super::emulator::EmulatorArgs;
use super::flutter::FlutterArgs;
use super::psdk::PsdkArgs;
use super::sdk::SdkArgs;

/// Классическая командная строка
#[derive(Args)]
#[command(arg_required_else_help = true)]
pub struct CliArgs {
    #[command(subcommand)]
    command: Option<CliCommands>,

    /// Показать это сообщение и выйти
    #[clap(short='h', long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,
}

#[derive(Subcommand)]
pub enum CliCommands {
    /// Работа с эмуляторами
    Emulator(EmulatorArgs),
    /// Работа с Flutter SDK
    Flutter(FlutterArgs),
    /// Работа с Platform SDK
    Psdk(PsdkArgs),
    /// Работа с Аврора SDK
    Sdk(SdkArgs),
}

/// Handling interface events
pub fn run(arg: CliArgs) {
    match arg.command.unwrap() {
        CliCommands::Emulator(arg) => super::emulator::run(arg),
        CliCommands::Flutter(arg) => super::flutter::run(arg),
        CliCommands::Psdk(arg) => super::psdk::run(arg),
        CliCommands::Sdk(arg) => super::sdk::run(arg),
    }
}
