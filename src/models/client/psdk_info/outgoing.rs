use serde::{Deserialize, Serialize};

use crate::models::{
    client::outgoing::{DataOutgoing, TraitOutgoing},
    psdk::model::PsdkModel,
    TraitModel,
};

use super::incoming::PsdkInfoIncoming;

#[derive(Serialize, Deserialize, Clone)]
pub struct PsdkInfoOutgoing {
    model: PsdkModel,
}

impl PsdkInfoOutgoing {
    pub fn new(model: PsdkModel) -> Box<PsdkInfoOutgoing> {
        Box::new(Self { model })
    }
}

impl TraitOutgoing for PsdkInfoOutgoing {
    fn print(&self) {
        self.model.print();
    }

    fn to_string(&self) -> String {
        DataOutgoing::serialize(PsdkInfoIncoming::name(), self.clone())
    }
}
