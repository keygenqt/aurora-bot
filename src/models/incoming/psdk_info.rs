use crate::models::{
    outgoing::{psdk_info::PsdkInfoOutgoing, Outgoing, OutgoingType},
    psdk::model::PsdkModel,
};

use super::{Incoming, TraitIncoming};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PsdkInfoIncoming {}

impl PsdkInfoIncoming {
    pub fn new() -> Incoming {
        Incoming::PsdkInfo(Self {})
    }
}

impl TraitIncoming for PsdkInfoIncoming {
    fn name() -> String {
        "PsdkInfo".into()
    }

    async fn run(&self, _: OutgoingType) -> Outgoing {
        PsdkInfoOutgoing::new(PsdkModel::search().await)
    }
}
