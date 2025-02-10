use crate::models::{
    flutter::model::FlutterModel,
    outgoing::{flutter_info::FlutterInfoOutgoing, Outgoing, OutgoingType},
};

use super::{Incoming, TraitIncoming};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct FlutterInfoIncoming {}

impl FlutterInfoIncoming {
    pub fn new() -> Incoming {
        Incoming::FlutterInfo(Self {})
    }
}

impl TraitIncoming for FlutterInfoIncoming {
    fn name() -> String {
        "FlutterInfo".into()
    }

    async fn run(&self, _: OutgoingType) -> Outgoing {
        FlutterInfoOutgoing::new(FlutterModel::search().await)
    }
}
