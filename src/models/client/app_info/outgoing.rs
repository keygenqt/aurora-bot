use serde::{Deserialize, Serialize};

use crate::{
    models::client::outgoing::{DataOutgoing, TraitOutgoing},
    tools::constants,
};

use super::incoming::AppInfoIncoming;

#[derive(Serialize, Deserialize, Clone)]
pub struct AppInfoOutgoing {
    version: String,
    api_version: String,
}

impl AppInfoOutgoing {
    pub fn new() -> Box<AppInfoOutgoing> {
        Box::new(Self {
            version: constants::VERSION_APP.to_string(),
            api_version: constants::VERSION_API.to_string(),
        })
    }
}

impl TraitOutgoing for AppInfoOutgoing {
    fn print(&self) {
        println!("aurora-bot v{} (api: v{})", self.version, self.api_version)
    }

    fn to_string(&self) -> String {
        DataOutgoing::serialize(AppInfoIncoming::name(), self.clone())
    }
}
