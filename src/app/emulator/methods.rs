use crate::{
    app::api::{
        enums::{ClientState, SendType},
        outgoing::Outgoing,
    },
    models::{emulator::model::EmulatorModel, incoming::EmulatorStartIncoming},
    utils::methods,
};

/// Start emulator
pub async fn emulator_start(
    _: &EmulatorStartIncoming,
    send_type: &SendType,
) -> Result<Outgoing, Box<dyn std::error::Error>> {
    // Send state search emulators
    methods::send_state(&Outgoing::emulator_start_state(1), send_type);
    // Search emulators
    let emulators = EmulatorModel::search()?;
    // Get first emulator, multiselect for the future
    if let Some(emulator) = emulators.iter().next() {
        if emulator.is_running {
            // Get emulator connect session
            let emulator = emulator.session_user().await?;
            // Close connect
            emulator.close().await?;
            // Is emulator already running
            return Ok(Outgoing::emulator_start(
                ClientState::Info,
                emulator.os_name,
            ));
        } else {
            // Send state about start
            methods::send_state(&Outgoing::emulator_start_state(2), send_type);
            // Start emulator
            emulator.start().await?;
            // Send state about connect
            methods::send_state(&Outgoing::emulator_start_state(3), send_type);
            // Get emulator connect session
            let emulator = emulator.session_user().await?;
            // Close connect
            emulator.close().await?;
            // All ok
            return Ok(Outgoing::emulator_start(
                ClientState::Success,
                emulator.os_name,
            ));
        }
    }
    // Ok - but emulators non found
    Ok(Outgoing::emulator_start(
        ClientState::Error,
        "эмуляторы не найдены".into(),
    ))
}
