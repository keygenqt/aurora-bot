use serde::Deserialize;
use serde::Serialize;

use crate::feature::outgoing::DataOutgoing;
use crate::feature::outgoing::TraitOutgoing;
use crate::models::TraitModel;
use crate::models::psdk_available::model::PsdkAvailableModel;

use super::incoming::PsdkAvailableIncoming;

#[derive(Serialize, Deserialize, Clone)]
pub struct PsdkAvailableOutgoing {
    model: PsdkAvailableModel,
}

impl PsdkAvailableOutgoing {
    pub fn new(model: PsdkAvailableModel) -> Box<PsdkAvailableOutgoing> {
        Box::new(Self { model })
    }
}

impl TraitOutgoing for PsdkAvailableOutgoing {
    fn print(&self) {
        self.model.print();
    }

    fn to_json(&self) -> String {
        DataOutgoing::serialize(PsdkAvailableIncoming::name(), self.clone())
    }
}
