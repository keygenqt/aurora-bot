use crate::service::requests::methods;
use crate::utils::macros::{print_error, print_info};
use crate::utils::single;

/// Handling interface events
pub async fn run(search: Option<Vec<String>>) {
    match search {
        Some(search) => match methods::get_search(single::get_request(), search.join(" ")).await {
            Ok(value) => value.print(),
            Err(error) => print_info!(error),
        },
        None => print_error!("введите вопрос в свободной форме"),
    }
}
