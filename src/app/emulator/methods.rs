use std::time::Duration;

use tokio::time::sleep;

use crate::{
    app::api::{
        enums::CommandType, incoming::models::EmulatorStartIncoming, outgoing::models::Outgoing,
    },
    utils::methods,
};

/// Update settings emulator
async fn emulator_refresh() {
    // @todo demo sleep
    sleep(Duration::from_millis(1000)).await;
}

/// Get emulator
async fn emulator_get() {
    // @todo demo sleep
    sleep(Duration::from_millis(1000)).await;
}

/// Start emulator
pub async fn emulator_start(
    _: &EmulatorStartIncoming,
    command: &CommandType,
    callback: Option<fn(&Outgoing)>,
) -> Result<Outgoing, Box<dyn std::error::Error>> {
    //////////// Send state ////////////
    methods::send_state(&Outgoing::emulator_start_state(1), command, callback);

    ////////// Run any methods /////////
    emulator_refresh().await;
    emulator_get().await;

    //////////// Send state ////////////
    methods::send_state(&Outgoing::emulator_start_state(2), command, callback);

    ////////// Run any methods /////////
    sleep(Duration::from_millis(1000)).await;

    //////////// Send done /////////////
    Ok(Outgoing::emulator_start())
}
