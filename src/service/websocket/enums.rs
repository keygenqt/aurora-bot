use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum MessageState {
    Success,
    Error,
    Info,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub enum MessageKey {
    Empty,
    Connection,
    PsdkInstalled,
    PsdkAvailable,
    AppInfo,
    EmulatorStart,
}
