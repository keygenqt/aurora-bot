use crate::{
    models::client::outgoing::OutgoingType,
    tools::{macros::print_error, single},
};

/// Handling interface events
pub fn run(command: Option<Vec<String>>) {
    match command {
        Some(command) => match single::get_request().get_command(command.join(" ")) {
            Ok(incoming) => incoming.run(OutgoingType::Cli).print(),
            Err(_) => print_error!("что-то пошло не так, попробуйте выполнить позже"),
        },
        None => print_error!("введите команду в свободной форме"),
    }
}
