use crate::{
    service::requests::methods,
    utils::{
        macros::{print_error, print_serde, print_info},
        single,
    },
};

/// Handling interface events
pub async fn run(command: Option<Vec<String>>) {
    match command {
        Some(command) => match methods::get_command(single::get_request(), command.join(" ")).await
        {
            Ok(value) => print_serde!(value),
            Err(error) => print_info!(error),
        },
        None => print_error!("введите команду в свободной форме"),
    }
}
