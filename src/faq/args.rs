use crate::feature::outgoing::TraitOutgoing;
use crate::tools::macros::print_error;
use crate::tools::single;

/// Handling interface events
pub fn run(search: Vec<String>) {
    match single::get_request().get_search(search.join(" ")) {
        Ok(value) => value.print(),
        Err(_) => print_error!("что-то пошло не так, попробуйте выполнить позже"),
    }
}
