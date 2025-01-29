use crate::{
    app::api::{enums::SendType, handler::handler_incoming},
    service::requests::methods,
    utils::{
        macros::{print_error, print_info},
        methods::print_outgoing,
        single,
    },
};

/// Handling interface events
pub async fn run(command: Option<Vec<String>>) {
    match command {
        Some(command) => match methods::get_command(single::get_request(), command.join(" ")).await
        {
            Ok(incoming) => match handler_incoming(&incoming, SendType::Cli).await {
                Ok(outgoing) => print_outgoing(&outgoing),
                Err(error) => print_error!(error),
            },
            Err(error) => print_info!(error),
        },
        None => print_error!("введите команду в свободной форме"),
    }
}
