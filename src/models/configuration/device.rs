use serde::{Deserialize, Serialize};
use crate::models::emulator::model::EmulatorType;

#[derive(Serialize, Deserialize, Clone)]
pub struct EmulatorConfiguration {
    pub emulator_type: EmulatorType,
}
