use serde::Deserialize;
use serde::Serialize;

use crate::feature::outgoing::DataOutgoing;
use crate::feature::outgoing::TraitOutgoing;
use crate::models::TraitModel;
use crate::models::sdk_available::model::SdkAvailableModel;

use super::incoming::SdkAvailableIncoming;

#[derive(Serialize, Deserialize, Clone)]
pub struct SdkAvailableOutgoing {
    model: SdkAvailableModel,
}

impl SdkAvailableOutgoing {
    pub fn new(model: SdkAvailableModel) -> Box<SdkAvailableOutgoing> {
        Box::new(Self { model })
    }
}

impl TraitOutgoing for SdkAvailableOutgoing {
    fn print(&self) {
        self.model.print();
    }

    fn to_json(&self) -> String {
        DataOutgoing::serialize(SdkAvailableIncoming::name(), self.clone())
    }
}
