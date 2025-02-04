use clap::Args;

use crate::service::dbus::server::ServerDbus;
use crate::utils::{
    macros::{print_error, print_info, print_success},
    single,
};

#[derive(Debug, Args)]
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
