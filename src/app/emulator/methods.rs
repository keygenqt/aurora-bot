use crate::{
    app::api::{
        enums::{ClientState, SendType},
        outgoing::Outgoing,
    },
    models::{emulator::EmulatorModel, incoming::EmulatorStartIncoming},
    utils::methods,
};

/// Start emulator
pub async fn emulator_start(
    _: &EmulatorStartIncoming,
    send_type: &SendType,
) -> Result<Outgoing, Box<dyn std::error::Error>> {
    // Search emulators
    methods::send_state(&Outgoing::emulator_start_state(1), send_type);
    let emulators = EmulatorModel::search()?;
    // @todo multiselect
    if let Some(emulator) = emulators.iter().next() {
        methods::send_state(&Outgoing::emulator_start_state(2), send_type);
        let result = emulator.start().await?;
        return Ok(result);
    }
    Ok(Outgoing::emulator_start(ClientState::Error))
}
