use crate::{
    models::{incoming::Incoming, outgoing::OutgoingType},
    utils::{
        macros::{print_error, print_info},
        single,
    },
};

/// Handling interface events
pub async fn run(command: Option<Vec<String>>) {
    match command {
        Some(command) => match single::get_request().get_command(command.join(" ")).await {
            Ok(incoming) => Incoming::handler(incoming, OutgoingType::Cli).await.print(),
            Err(error) => print_info!(error),
        },
        None => print_error!("введите команду в свободной форме"),
    }
}
