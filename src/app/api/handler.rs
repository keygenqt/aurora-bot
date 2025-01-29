use crate::app::emulator::methods::emulator_start;

use super::enums::SendType;
use super::incoming::models::Incoming;
use super::outgoing::models::Outgoing;

pub async fn handler_incoming(
    value: &Incoming,
    send_type: SendType,
) -> Result<Outgoing, Box<dyn std::error::Error>> {
    match value {
        Incoming::Connection(incoming) => Ok(Outgoing::connection(format!("{}", incoming.message))),
        Incoming::AppInfo(_) => Ok(Outgoing::app_info()),
        Incoming::EmulatorStart(incoming) => emulator_start(incoming, &send_type).await,
    }
}
