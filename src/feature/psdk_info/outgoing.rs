use serde::Deserialize;
use serde::Serialize;

use crate::feature::outgoing::DataOutgoing;
use crate::feature::outgoing::TraitOutgoing;
use crate::models::TraitModel;
use crate::models::psdk_installed::model::PsdkInstalledModel;

use super::incoming::PsdkInfoIncoming;

#[derive(Serialize, Deserialize, Clone)]
pub struct PsdkInfoOutgoing {
    model: PsdkInstalledModel,
}

impl PsdkInfoOutgoing {
    pub fn new(model: PsdkInstalledModel) -> Box<PsdkInfoOutgoing> {
        Box::new(Self { model })
    }
}

impl TraitOutgoing for PsdkInfoOutgoing {
    fn print(&self) {
        self.model.print();
    }

    fn to_json(&self) -> String {
        DataOutgoing::serialize(PsdkInfoIncoming::name(), self.clone())
    }
}
