use std::time::Duration;

use tokio::time::sleep;

use crate::{
    app::api::{
        enums::SendType, incoming::{self, models::EmulatorStartIncoming}, outgoing::models::Outgoing,
    },
    utils::{macros::print_serde, methods},
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
    incoming: &EmulatorStartIncoming,
    send_type: &SendType,
) -> Result<Outgoing, Box<dyn std::error::Error>> {
    ////////// Print incoming //////////
    print_serde!(incoming);

    //////////// Send state ////////////
    methods::send_state(&Outgoing::emulator_start_state(1), send_type);

    ////////// Run any methods /////////
    emulator_refresh().await;
    emulator_get().await;

    //////////// Send state ////////////
    methods::send_state(&Outgoing::emulator_start_state(2), send_type);

    ////////// Run any methods /////////
    sleep(Duration::from_millis(1000)).await;

    //////////// Send done /////////////
    Ok(Outgoing::emulator_start())
}
