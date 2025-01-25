use clap::Args;

use crate::{
    service::websocket::{
        enums::{MessageKey, MessageState},
        outgoing::WebsocketMessage,
    },
    utils::{
        macros::{print_error, print_info, print_serde, print_success},
        single,
    },
};

#[derive(Debug, Args)]
#[command()]
#[command(arg_required_else_help = true)]
#[group(multiple = false)]
pub struct SvcArgs {
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
    if arg.connect {
        let websocket = single::get_websocket();
        if !websocket.is_none() {
            match websocket
                .unwrap()
                .connect(|incoming| {
                    print_serde!(incoming);
                    match incoming.key {
                        MessageKey::AppInfo => WebsocketMessage::app_info(),
                        MessageKey::EmulatorStart => WebsocketMessage::emulator_start(
                            MessageState::Success,
                            "Эмулятор успешно запущен!",
                        ),
                        _ => WebsocketMessage::empty(),
                    }
                })
                .await
            {
                Ok(_) => print_info!("соединение закрыто"),
                Err(_) => print_error!("соединение не установлено"),
            }
        }
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
                Err(_) => print_error!(
                    "не удалось подключиться, проверьте соединение и актуальность токена"
                ),
            }
        }
    }
}