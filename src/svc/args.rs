use crate::feature::app_auth_login::incoming::AppAuthLoginIncoming;
use crate::feature::app_auth_logout::incoming::AppAuthLogoutIncoming;
use crate::feature::device_sync::incoming::DeviceSyncIncoming;
use crate::feature::emulator_sync::incoming::EmulatorSyncIncoming;
use crate::feature::flutter_sync::incoming::FlutterSyncIncoming;
use crate::feature::incoming::TraitIncoming;
use crate::feature::outgoing::OutgoingType;
use crate::feature::psdk_sync::incoming::PsdkSyncIncoming;
use crate::feature::sdk_sync::incoming::SdkSyncIncoming;
use crate::models::psdk_installed::model::PsdkInstalledModel;
use crate::service::command;
use crate::service::dbus::server::ServerDbus;
use crate::tools::macros::crash;
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
    /// Запустить D-Bus сервер
    #[arg(long, value_name = "suffix")]
    dbus_suffix: Option<String>,

    /// Подключиться к удаленному сервису
    #[arg(short, long, default_value_t = false)]
    connect: bool,

    /// Авторизация с использованию токена
    #[arg(short, long, value_name = "token")]
    auth: Option<String>,

    /// Остановить сервисы и удалить сессию подключения
    #[arg(short, long, default_value_t = false)]
    logout: bool,

    /// Обновить sudoers для Platform SDK
    #[arg(short, long, default_value_t = false)]
    permissions: bool,

    /// Поиск и синхронизация
    #[command(subcommand)]
    sync: Option<SyncCommands>,

    /// Показать это сообщение и выйти
    #[clap(short='h', long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,
}

#[derive(Subcommand)]
pub enum SyncCommands {
    /// Синхронизация данных
    #[command(short_flag = 's')]
    Sync(SyncArgs),
}

#[derive(Args)]
#[command()]
#[command(arg_required_else_help = true)]
#[group(multiple = false)]
pub struct SyncArgs {
    /// Поиск и синхронизация устройств
    #[arg(short, long, default_value_t = false)]
    device: bool,

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

    /// Показать это сообщение и выйти
    #[clap(short='h', long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,
}

/// Handling interface events
pub fn run(arg: SvcArgs) {
    if arg.dbus {
        match ServerDbus::run(None) {
            Ok(_) => print_info!("соединение закрыто"),
            Err(_) => print_error!("не удалось активировать сервер"),
        }
        return;
    }
    if let Some(suffix) = arg.dbus_suffix {
        match ServerDbus::run(Some(suffix)) {
            Ok(_) => print_info!("соединение закрыто"),
            Err(_) => print_error!("не удалось активировать сервер"),
        }
        return;
    }
    if arg.connect {
        match single::get_websocket().run() {
            Ok(_) => print_info!("соединение закрыто"),
            Err(_) => print_error!("соединение не установлено"),
        }
        return;
    }
    if arg.logout {
        AppAuthLogoutIncoming::new().run(OutgoingType::Cli).print();
        return;
    }
    if arg.permissions {
        print_info!("поиск Platform SDK в системе");
        let models = match PsdkInstalledModel::search_full_without_targets() {
            Ok(value) => value,
            Err(error) => crash!(error),
        };
        print_info!("обновление записи sudoers");
        match command::psdk::add_sudoers_chroot_access(&models) {
            Ok(_) => print_success!("запись sudoers успешно обновлена"),
            Err(error) => print_error!(error),
        }
        return;
    }
    if let Some(token) = arg.auth {
        AppAuthLoginIncoming::new(token).run(OutgoingType::Cli).print();
        return;
    }
    if let Some(sync) = arg.sync {
        match sync {
            SyncCommands::Sync(arg) => {
                if arg.device || arg.all {
                    DeviceSyncIncoming::new().run(OutgoingType::Cli).print();
                }
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
