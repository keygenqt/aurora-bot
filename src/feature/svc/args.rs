use clap::Args;

#[derive(Debug, Args)]
#[command()]
#[command(arg_required_else_help = true)]
pub struct SvcArgs {
    /// Запустить сервисы в фоновом режиме
    #[arg(short, long, default_value_t = false)]
    run: bool,

    /// Остановить сервисы
    #[arg(short, long, default_value_t = false)]
    stop: bool,

    /// Авторизация с использованию токена
    #[arg(short, long, value_name = "hash")]
    token: Option<String>,
}

/// Handling interface events
pub async fn run(arg: SvcArgs) {
    println!("{:?}", arg.run);
    println!("{:?}", arg.stop);
    println!("{:?}", arg.token);
}
