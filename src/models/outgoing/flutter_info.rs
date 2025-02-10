use serde::{Deserialize, Serialize};

use crate::models::{flutter::model::FlutterModel, TraitModel};

use super::{Outgoing, TraitOutgoing};

#[derive(Serialize, Deserialize, Clone)]
pub struct FlutterInfoOutgoing {
    data: Vec<FlutterModel>,
}

impl FlutterInfoOutgoing {
    pub fn new(data: Vec<FlutterModel>) -> Outgoing {
        Outgoing::FlutterInfo(Self { data })
    }
}

impl TraitOutgoing for FlutterInfoOutgoing {
    fn print(&self) {
        <dyn TraitModel>::print_list(self.data.clone());
    }
}
