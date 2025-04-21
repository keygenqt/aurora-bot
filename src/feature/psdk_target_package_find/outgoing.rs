use serde::Deserialize;
use serde::Serialize;

use crate::feature::outgoing::DataOutgoing;
use crate::feature::outgoing::TraitOutgoing;
use crate::models::TraitModel;
use crate::models::psdk_target_package::model::PsdkTargetPackageModel;
use crate::tools::macros::print_info;
use crate::tools::macros::tr;

use super::incoming::PsdkTargetPackageFindIncoming;

#[derive(Serialize, Deserialize, Clone)]
pub struct PsdkTargetPackageFindOutgoing {
    packages: Vec<PsdkTargetPackageModel>,
}

impl PsdkTargetPackageFindOutgoing {
    pub fn new(packages: Vec<PsdkTargetPackageModel>) -> Box<PsdkTargetPackageFindOutgoing> {
        Box::new(Self { packages })
    }
}

impl TraitOutgoing for PsdkTargetPackageFindOutgoing {
    fn print(&self) {
        if self.packages.is_empty() {
            let out = tr!("ничего не найдено");
            print_info!(out);
            return;
        }
        for item in &self.packages {
            item.print();
        }
    }

    fn to_json(&self) -> String {
        DataOutgoing::serialize(PsdkTargetPackageFindIncoming::name(), self.clone())
    }
}
