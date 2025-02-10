use serde::{Deserialize, Serialize};

use crate::models::{psdk::model::PsdkModel, TraitModel};

use super::{Outgoing, TraitOutgoing};

#[derive(Serialize, Deserialize, Clone)]
pub struct PsdkInfoOutgoing {
    data: Vec<PsdkModel>,
}

impl PsdkInfoOutgoing {
    pub fn new(data: Vec<PsdkModel>) -> Outgoing {
        Outgoing::PsdkInfo(Self { data })
    }
}

impl TraitOutgoing for PsdkInfoOutgoing {
    fn print(&self) {
        <dyn TraitModel>::print_list(self.data.clone());
    }
}
