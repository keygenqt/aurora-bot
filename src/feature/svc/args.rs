use crate::models::configuration::device::DeviceConfig;
use crate::models::configuration::emulator::EmulatorConfig;
use crate::models::configuration::flutter::FlutterConfig;
use crate::models::configuration::psdk::PsdkConfig;
use crate::models::configuration::sdk::SdkConfig;
use crate::service::dbus::server::ServerDbus;
use crate::utils::{
    macros::{print_error, print_info, print_success},
    single,
};
use clap::{Args, Subcommand};

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
    /// Поиск и синхронизация эмуляторов
    #[arg(short, long, default_value_t = false)]
    emulator: bool,

    /// Поиск и синхронизация устройств
    #[arg(short, long, default_value_t = false)]
    device: bool,

    /// Поиск и синхронизация Flutter SDK
    #[arg(short, long, default_value_t = false)]
    flutter: bool,

    /// Поиск и синхронизация Аврора Platform SDK
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
pub async fn run(arg: SvcArgs) {
    if arg.dbus {
        match ServerDbus::run().await {
            Ok(_) => print_info!("соединение закрыто"),
            Err(_) => print_error!("не удалось активировать сервер"),
        }
    } else if arg.connect {
        match single::get_websocket().run().await {
            Ok(_) => print_info!("соединение закрыто"),
            Err(_) => print_error!("соединение не установлено"),
        }
    } else if arg.logout {
        match single::get_request().logout() {
            Ok(_) => print_success!("сессия удалена успешно"),
            Err(_) => print_error!("сессия не найдена"),
        }
    } else if let Some(sync) = arg.sync {
        match sync {
            SyncCommands::Sync(arg) => {
                if arg.device {
                    DeviceConfig::search().await.save();
                } else if arg.emulator {
                    EmulatorConfig::search().await.save();
                } else if arg.flutter {
                    FlutterConfig::search().await.save();
                } else if arg.psdk {
                    PsdkConfig::search().await.save();
                } else if arg.sdk {
                    SdkConfig::search().await.save();
                } else {
                    DeviceConfig::search().await.save();
                    EmulatorConfig::search().await.save();
                    PsdkConfig::search().await.save();
                    SdkConfig::search().await.save();
                }
            }
        }
    } else {
        match single::get_request()
            .auth_ping_token(arg.auth.unwrap())
            .await
        {
            Ok(_) => print_success!("авторизация выполнена успешно"),
            Err(_) => {
                print_error!("не удалось подключиться, проверьте соединение и актуальность токена")
            }
        }
    }
}
