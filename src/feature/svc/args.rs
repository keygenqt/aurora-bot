use clap::Args;

use crate::utils::{macros::{print_error, print_success}, single};

#[derive(Debug, Args)]
#[command()]
#[command(arg_required_else_help = true)]
#[group(multiple = false)]
pub struct SvcArgs {
    /// Запустить сервисы в фоновом режиме
    #[arg(short, long, default_value_t = false)]
    run: bool,

    /// Остановить активные сервисы
    #[arg(short, long, default_value_t = false)]
    stop: bool,

    /// Авторизация с использованию токена
    #[arg(short, long, value_name = "token")]
    auth: Option<String>,

    /// Остановить сервисы и удалить сессию подключения
    #[arg(short, long, default_value_t = false)]
    logout: bool,
}

/// Handling interface events
pub async fn run(arg: SvcArgs) {
    if arg.run {
        println!("run")
    } else if arg.stop {
        println!("stop")
    } else if arg.logout {
        let request = single::get_request();
        if !request.is_none() {
            match request.unwrap().logout() {
                Ok(_) => print_success!("сессия удалена успешно"),
                Err(_) => print_error!("сессия не найдена"),
            }
        }
    } else {
        let request = single::get_request();
        if !request.is_none() {
            match request.unwrap().auth_ping_token(arg.auth.unwrap()).await {
                Ok(_) => print_success!("авторизация выполнена успешно"),
                Err(_) => print_error!("не удалось подключиться, проверьте соединение и актуальность токена"),
            }
        }
    }
}
