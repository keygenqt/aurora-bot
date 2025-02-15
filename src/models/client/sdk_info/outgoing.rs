use serde::{Deserialize, Serialize};

use crate::models::{
    client::outgoing::{DataOutgoing, TraitOutgoing},
    sdk::model::SdkModel,
    TraitModel,
};

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

    fn to_string(&self) -> String {
        DataOutgoing::serialize(SdkInfoIncoming::name(), self.clone())
    }
}
