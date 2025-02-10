use serde::{Deserialize, Serialize};

use crate::models::{
    emulator::model::EmulatorModel,
    outgoing::{
        emulator_start::EmulatorStartOutgoing, emulator_start_state::EmulatorStartStateOutgoing,
        Outgoing, OutgoingState, OutgoingType,
    },
};

use super::{Incoming, TraitIncoming};

#[derive(Serialize, Deserialize, Clone)]
pub struct EmulatorStartIncoming {}

impl EmulatorStartIncoming {
    pub fn new() -> Incoming {
        Incoming::EmulatorStart(Self {})
    }
}

impl TraitIncoming for EmulatorStartIncoming {
    fn name() -> String {
        "EmulatorStart".into()
    }

    async fn run(&self, send_type: OutgoingType) -> Outgoing {
        async fn _exec(send_type: OutgoingType) -> Result<Outgoing, Box<dyn std::error::Error>> {
            // Send state search emulators
            EmulatorStartStateOutgoing::new_search().send(&send_type);
            // Search emulators
            let emulators = EmulatorModel::search().await;
            // Get first emulator, multiselect for the future
            if let Some(emulator) = emulators.iter().next() {
                return if emulator.is_running {
                    // Get emulator connect session
                    let emulator = emulator.session_user().await?;
                    // Close connect
                    emulator.close().await?;
                    // Is emulator already running
                    Ok(EmulatorStartOutgoing::new(
                        OutgoingState::Info,
                        emulator.os_name,
                    ))
                } else {
                    // Send state about start
                    EmulatorStartStateOutgoing::new_start().send(&send_type);
                    // Start emulator
                    emulator.start().await?;
                    // Send state about connect
                    EmulatorStartStateOutgoing::new_loading().send(&send_type);
                    // Get emulator connect session
                    let emulator = emulator.session_user().await?;
                    // Close connect
                    emulator.close().await?;
                    // All ok
                    Ok(EmulatorStartOutgoing::new(
                        OutgoingState::Success,
                        emulator.os_name,
                    ))
                };
            }
            // Ok - but emulators non found
            Ok(EmulatorStartOutgoing::new(
                OutgoingState::Error,
                "эмуляторы не найдены".into(),
            ))
        }
        _exec(send_type).await.unwrap_or_else(|_| {
            EmulatorStartOutgoing::new(OutgoingState::Error, "ошибка запуска эмулятора".into())
        })
    }
}
