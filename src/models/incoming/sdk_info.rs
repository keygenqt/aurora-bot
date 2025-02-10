use crate::models::{
    outgoing::{sdk_info::SdkInfoOutgoing, Outgoing, OutgoingType},
    sdk::model::SdkModel,
};

use super::{Incoming, TraitIncoming};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct SdkInfoIncoming {}

impl SdkInfoIncoming {
    pub fn new() -> Incoming {
        Incoming::SdkInfo(Self {})
    }
}

impl TraitIncoming for SdkInfoIncoming {
    fn name() -> String {
        "SdkInfo".into()
    }

    async fn run(&self, _: OutgoingType) -> Outgoing {
        SdkInfoOutgoing::new(SdkModel::search().await)
    }
}
