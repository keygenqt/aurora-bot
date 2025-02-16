use crate::models::client::emulator_sync::incoming::EmulatorSyncIncoming;
use crate::models::client::flutter_sync::incoming::FlutterSyncIncoming;
use crate::models::client::incoming::TraitIncoming;
use crate::models::client::outgoing::OutgoingType;
use crate::models::client::psdk_sync::incoming::PsdkSyncIncoming;
use crate::models::client::sdk_sync::incoming::SdkSyncIncoming;
use crate::service::dbus::server::ServerDbus;
use crate::tools::macros::print_error;
use crate::tools::macros::print_info;
use crate::tools::macros::print_success;
use crate::tools::single;
use clap::Args;
use clap::Subcommand;

#[derive(Args)]
#[command()]
#[command(arg_required_else_help = true)]
#[group(multiple = false)]
pub struct SvcArgs {
    /// Запустить D-Bus сервер
    #[arg(short, long, default_value_t = false)]
    dbus: bool,

    /// Подключиться к удаленному сервису
    #[arg(short, long, default_value_t = false)]
    connect: bool,

    /// Авторизация с использованию токена
    #[arg(short, long, value_name = "token")]
    auth: Option<String>,

    /// Остановить сервисы и удалить сессию подключения
    #[arg(short, long, default_value_t = false)]
    logout: bool,

    /// Поиск и синхронизация
    #[command(subcommand)]
    sync: Option<SyncCommands>,
}

#[derive(Subcommand)]
pub enum SyncCommands {
    /// Синхронизация данных
    Sync(SyncArgs),
}

#[derive(Args)]
#[command()]
#[command(arg_required_else_help = true)]
#[group(multiple = false)]
pub struct SyncArgs {
    // /// Поиск и синхронизация устройств
    // #[arg(short, long, default_value_t = false)]
    // device: bool,
    /// Поиск и синхронизация эмуляторов
    #[arg(short, long, default_value_t = false)]
    emulator: bool,

    /// Поиск и синхронизация Flutter SDK
    #[arg(short, long, default_value_t = false)]
    flutter: bool,

    /// Поиск и синхронизация Platform SDK
    #[arg(short, long, default_value_t = false)]
    psdk: bool,

    /// Поиск и синхронизация Аврора SDK
    #[arg(short, long, default_value_t = false)]
    sdk: bool,

    /// Синхронизировать все
    #[arg(short, long, default_value_t = false)]
    all: bool,
}

/// Handling interface events
pub fn run(arg: SvcArgs) {
    if arg.dbus {
        match ServerDbus::run() {
            Ok(_) => print_info!("соединение закрыто"),
            Err(_) => print_error!("не удалось активировать сервер"),
        }
    }
    if arg.connect {
        match single::get_websocket().run() {
            Ok(_) => print_info!("соединение закрыто"),
            Err(_) => print_error!("соединение не установлено"),
        }
    }
    if arg.logout {
        match single::get_request().logout() {
            Ok(_) => print_success!("сессия удалена успешно"),
            Err(_) => print_error!("сессия не найдена"),
        }
    }
    if let Some(token) = arg.auth {
        match single::get_request().auth_ping_token(token) {
            Ok(_) => print_success!("авторизация выполнена успешно"),
            Err(_) => print_error!("авторизация по токену не удалась"),
        }
    }
    if let Some(sync) = arg.sync {
        match sync {
            SyncCommands::Sync(arg) => {
                if arg.emulator || arg.all {
                    EmulatorSyncIncoming::new().run(OutgoingType::Cli).print();
                }
                if arg.flutter || arg.all {
                    FlutterSyncIncoming::new().run(OutgoingType::Cli).print();
                }
                if arg.psdk || arg.all {
                    PsdkSyncIncoming::new().run(OutgoingType::Cli).print();
                }
                if arg.sdk || arg.all {
                    SdkSyncIncoming::new().run(OutgoingType::Cli).print();
                }
            }
        }
    }
}
