use super::{Incoming, TraitIncoming};
use crate::models::emulator::model::EmulatorModel;
use crate::models::outgoing::emulator_close::OutgoingEmulatorClose;
use crate::models::outgoing::emulator_close_state::OutgoingEmulatorCloseState;
use crate::models::outgoing::{Outgoing, OutgoingState, OutgoingType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct IncomingEmulatorClose {}

impl IncomingEmulatorClose {
    pub fn new() -> Incoming {
        Incoming::EmulatorClose(Self {})
    }
}

impl TraitIncoming for IncomingEmulatorClose {
    fn name() -> String {
        "EmulatorClose".into()
    }

    async fn run(&self, send_type: OutgoingType) -> Outgoing {
        async fn exec(send_type: OutgoingType) -> Result<Outgoing, Box<dyn std::error::Error>> {
            // Send state search emulators
            OutgoingEmulatorCloseState::new_search().send(&send_type);
            // Search emulators
            let emulators = EmulatorModel::search()?;
            // Close all emulators
            let mut is_close = false;
            for emulator in emulators.iter() {
                if emulator.is_running {
                    // Send state about start
                    OutgoingEmulatorCloseState::new_close().send(&send_type);
                    emulator.close().await?;
                    is_close = true;
                }
            }
            if is_close {
                Ok(OutgoingEmulatorClose::new(OutgoingState::Success))
            } else {
                Ok(OutgoingEmulatorClose::new(OutgoingState::Info))
            }
        }
        exec(send_type)
            .await
            .unwrap_or_else(|_| OutgoingEmulatorClose::new(OutgoingState::Error))
    }
}
