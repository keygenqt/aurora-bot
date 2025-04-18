use serde::Deserialize;
use serde::Serialize;

use crate::feature::outgoing::DataOutgoing;
use crate::feature::outgoing::TraitOutgoing;
use crate::models::demo_app::model::DemoAppModel;
use crate::models::TraitModel;

use super::incoming::DemoAppInfoIncoming;

#[derive(Serialize, Deserialize, Clone)]
pub struct DemoAppInfoOutgoing {
    model: DemoAppModel,
}

impl DemoAppInfoOutgoing {
    pub fn new(model: DemoAppModel) -> Box<DemoAppInfoOutgoing> {
        Box::new(Self { model })
    }
}

impl TraitOutgoing for DemoAppInfoOutgoing {
    fn print(&self) {
        self.model.print();
    }

    fn to_json(&self) -> String {
        DataOutgoing::serialize(DemoAppInfoIncoming::name(), self.clone())
    }
}
