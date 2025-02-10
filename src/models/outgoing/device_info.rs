use serde::{Deserialize, Serialize};

use crate::models::{device::model::DeviceModel, TraitModel};

use super::{Outgoing, TraitOutgoing};

#[derive(Serialize, Deserialize, Clone)]
pub struct DeviceInfoOutgoing {
    data: Vec<DeviceModel>,
}

impl DeviceInfoOutgoing {
    pub fn new(data: Vec<DeviceModel>) -> Outgoing {
        Outgoing::DeviceInfo(Self { data })
    }
}

impl TraitOutgoing for DeviceInfoOutgoing {
    fn print(&self) {
        <dyn TraitModel>::print_list(self.data.clone());
    }
}
