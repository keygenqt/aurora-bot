use crate::app::emulator::methods::emulator_start;

use super::enums::CommandType;
use super::incoming::models::Incoming;
use super::outgoing::models::Outgoing;

pub async fn handler_incoming(
    value: &Incoming,
    command: CommandType,
    callback: Option<fn(&Outgoing)>,
) -> Result<Outgoing, Box<dyn std::error::Error>> {
    match value {
        Incoming::Connection(incoming) => Ok(Outgoing::connection(format!("{}", incoming.message))),
        Incoming::AppInfo(_) => Ok(Outgoing::app_info()),
        Incoming::EmulatorStart(incoming) => emulator_start(incoming, &command, callback).await,
    }
}
