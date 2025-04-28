use serde::Deserialize;
use serde::Serialize;

use crate::feature::outgoing::DataOutgoing;
use crate::feature::outgoing::TraitOutgoing;
use crate::models::TraitModel;
use crate::models::device::model::DeviceModel;

use super::incoming::DeviceInfoIncoming;

#[derive(Serialize, Deserialize, Clone)]
pub struct DeviceInfoOutgoing {
    model: DeviceModel,
}

impl DeviceInfoOutgoing {
    pub fn new(model: DeviceModel) -> Box<DeviceInfoOutgoing> {
        Box::new(Self { model })
    }
}

impl TraitOutgoing for DeviceInfoOutgoing {
    fn print(&self) {
        self.model.print();
    }

    fn to_json(&self) -> String {
        DataOutgoing::serialize(DeviceInfoIncoming::name(), self.clone())
    }
}
