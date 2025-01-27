use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum ClientState {
    Success,
    Error,
    Info,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub enum ClientKey {
    Connection,
    AppInfo,
    EmulatorStart,
    EmulatorStartState,
}

impl fmt::Display for ClientKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
