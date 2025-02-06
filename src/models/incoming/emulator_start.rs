use serde::{Deserialize, Serialize};

use crate::models::{
    emulator::model::EmulatorModel,
    outgoing::{
        emulator_start::OutgoingEmulatorStart, emulator_start_state::OutgoingEmulatorStartState,
        Outgoing, OutgoingState, OutgoingType,
    },
};

use super::{Incoming, TraitIncoming};

#[derive(Serialize, Deserialize, Clone)]
pub struct IncomingEmulatorStart {}

impl IncomingEmulatorStart {
    pub fn new() -> Incoming {
        Incoming::EmulatorStart(Self {})
    }
}

impl TraitIncoming for IncomingEmulatorStart {
    fn name() -> String {
        "EmulatorStart".into()
    }

    async fn run(&self, send_type: OutgoingType) -> Outgoing {
        async fn exec(send_type: OutgoingType) -> Result<Outgoing, Box<dyn std::error::Error>> {
            // Send state search emulators
            OutgoingEmulatorStartState::new_search().send(&send_type);
            // Search emulators
            let emulators = EmulatorModel::search()?;
            // Get first emulator, multiselect for the future
            if let Some(emulator) = emulators.iter().next() {
                return if emulator.is_running {
                    // Get emulator connect session
                    let emulator = emulator.session_user().await?;
                    // Close connect
                    emulator.close().await?;
                    // Is emulator already running
                    Ok(OutgoingEmulatorStart::new(
                        OutgoingState::Info,
                        emulator.os_name,
                    ))
                } else {
                    // Send state about start
                    OutgoingEmulatorStartState::new_start().send(&send_type);
                    // Start emulator
                    emulator.start().await?;
                    // Send state about connect
                    OutgoingEmulatorStartState::new_loading().send(&send_type);
                    // Get emulator connect session
                    let emulator = emulator.session_user().await?;
                    // Close connect
                    emulator.close().await?;
                    // All ok
                    Ok(OutgoingEmulatorStart::new(
                        OutgoingState::Success,
                        emulator.os_name,
                    ))
                };
            }
            // Ok - but emulators non found
            Ok(OutgoingEmulatorStart::new(
                OutgoingState::Error,
                "эмуляторы не найдены".into(),
            ))
        }
        exec(send_type).await.unwrap_or_else(|_| {
            OutgoingEmulatorStart::new(OutgoingState::Error, "ошибка запуска эмулятора".into())
        })
    }
}
