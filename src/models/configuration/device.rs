use crate::models::device::model::DeviceModel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct DeviceConfiguration {
    pub ip: String,
    pub host: String,
}

impl DeviceConfiguration {
    pub fn to_model(&self) -> DeviceModel {
        DeviceModel {
            ip: self.ip.clone(),
            host: self.host.clone(),
        }
    }
}
