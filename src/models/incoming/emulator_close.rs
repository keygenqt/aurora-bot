use super::{Incoming, TraitIncoming};
use crate::models::emulator::model::EmulatorModel;
use crate::models::outgoing::emulator_close::EmulatorCloseOutgoing;
use crate::models::outgoing::emulator_close_state::EmulatorCloseStateOutgoing;
use crate::models::outgoing::{Outgoing, OutgoingState, OutgoingType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct EmulatorCloseIncoming {}

impl EmulatorCloseIncoming {
    pub fn new() -> Incoming {
        Incoming::EmulatorClose(Self {})
    }
}

impl TraitIncoming for EmulatorCloseIncoming {
    fn name() -> String {
        "EmulatorClose".into()
    }

    async fn run(&self, send_type: OutgoingType) -> Outgoing {
        async fn _exec(send_type: OutgoingType) -> Result<Outgoing, Box<dyn std::error::Error>> {
            // Send state search emulators
            EmulatorCloseStateOutgoing::new_search().send(&send_type);
            // Search emulators
            let emulators = EmulatorModel::search().await?;
            // Close all emulators
            let mut is_close = false;
            for emulator in emulators.iter() {
                if emulator.is_running {
                    // Send state about start
                    EmulatorCloseStateOutgoing::new_close().send(&send_type);
                    emulator.close().await?;
                    is_close = true;
                }
            }
            if is_close {
                Ok(EmulatorCloseOutgoing::new(OutgoingState::Success))
            } else {
                Ok(EmulatorCloseOutgoing::new(OutgoingState::Info))
            }
        }
        _exec(send_type)
            .await
            .unwrap_or_else(|_| EmulatorCloseOutgoing::new(OutgoingState::Error))
    }
}
