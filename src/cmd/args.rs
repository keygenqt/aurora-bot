use crate::feature::outgoing::OutgoingType;
use crate::tools::macros::print_error;
use crate::tools::single;

/// Handling interface events
pub fn run(command: Vec<String>) {
    match single::get_request().get_command(command.join(" ")) {
        Ok(incoming) => incoming.run(OutgoingType::Cli).print(),
        Err(error) => print_error!(error),
    }
}
