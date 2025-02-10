use serde::{Deserialize, Serialize};

use crate::models::{sdk::model::SdkModel, TraitModel};

use super::{Outgoing, TraitOutgoing};

#[derive(Serialize, Deserialize, Clone)]
pub struct SdkInfoOutgoing {
    data: Vec<SdkModel>,
}

impl SdkInfoOutgoing {
    pub fn new(data: Vec<SdkModel>) -> Outgoing {
        Outgoing::SdkInfo(Self { data })
    }
}

impl TraitOutgoing for SdkInfoOutgoing {
    fn print(&self) {
        <dyn TraitModel>::print_list(self.data.clone());
    }
}
