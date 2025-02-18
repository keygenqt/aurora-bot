use serde::Deserialize;
use serde::Serialize;

use crate::models::client::outgoing::DataOutgoing;
use crate::models::client::outgoing::TraitOutgoing;
use crate::models::sdk::model::SdkModel;
use crate::models::TraitModel;

use super::incoming::SdkInfoIncoming;

#[derive(Serialize, Deserialize, Clone)]
pub struct SdkInfoOutgoing {
    model: SdkModel,
}

impl SdkInfoOutgoing {
    pub fn new(model: SdkModel) -> Box<SdkInfoOutgoing> {
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
