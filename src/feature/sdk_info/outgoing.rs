use serde::Deserialize;
use serde::Serialize;

use crate::feature::outgoing::DataOutgoing;
use crate::feature::outgoing::TraitOutgoing;
use crate::models::TraitModel;
use crate::models::sdk_installed::model::SdkInstalledModel;

use super::incoming::SdkInfoIncoming;

#[derive(Serialize, Deserialize, Clone)]
pub struct SdkInfoOutgoing {
    model: SdkInstalledModel,
}

impl SdkInfoOutgoing {
    pub fn new(model: SdkInstalledModel) -> Box<SdkInfoOutgoing> {
        Box::new(Self { model })
    }
}

impl TraitOutgoing for SdkInfoOutgoing {
    fn print(&self) {
        self.model.print();
    }

    fn to_json(&self) -> String {
        DataOutgoing::serialize(SdkInfoIncoming::name(), self.clone())
    }
}
