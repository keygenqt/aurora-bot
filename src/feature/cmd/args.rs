use crate::{
    app::api::{handler::handler_callback, outgoing::models::Outgoing},
    service::requests::methods,
    utils::{
        macros::{print_error, print_info, print_serde},
        single,
    },
};

/// Handling interface events
pub async fn run(command: Option<Vec<String>>) {
    match command {
        Some(command) => match methods::get_command(single::get_request(), command.join(" ")).await
        {
            Ok(value) => match handler_callback(value, |value| match value {
                // State callback
                Outgoing::EmulatorStartState(outgoing) => {
                    print_serde!(outgoing);
                }
                _ => {}
            })
            .await
            {
                // Done
                Some(outgoing) => match outgoing {
                    Outgoing::AppInfo(outgoing) => {
                        println!("aurora-bot {}", outgoing.version)
                    }
                    Outgoing::EmulatorStart(outgoing) => {
                        print_serde!(outgoing);
                    }
                    _ => {}
                },
                None => {}
            },
            Err(error) => print_info!(error),
        },
        None => print_error!("введите команду в свободной форме"),
    }
}
