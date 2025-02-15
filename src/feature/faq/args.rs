use crate::tools::{
    macros::{print_error, print_info},
    single,
};

/// Handling interface events
pub fn run(search: Option<Vec<String>>) {
    match search {
        Some(search) => match single::get_request().get_search(search.join(" ")) {
            Ok(value) => value.print(),
            Err(_) => print_info!("ничего не найдено, попробуйте переформулировать"),
        },
        None => print_error!("введите вопрос в свободной форме"),
    }
}
