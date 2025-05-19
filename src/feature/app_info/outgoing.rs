use serde::Deserialize;
use serde::Serialize;

use crate::feature::outgoing::DataOutgoing;
use crate::feature::outgoing::TraitOutgoing;
use crate::service::requests::client::ClientRequest;
use crate::tools::constants;

use super::incoming::AppInfoIncoming;

// @todo add is auth
#[derive(Serialize, Deserialize, Clone)]
pub struct AppInfoOutgoing {
    version: String,
    api_version: String,
    is_connect: bool,
}

impl AppInfoOutgoing {
    pub fn new() -> Box<AppInfoOutgoing> {
        let is_connect = match ClientRequest::new(Some(2)).get_user() {
            Ok(_) => true,
            Err(_) => false,
        };
        Box::new(Self {
            version: constants::VERSION_APP.to_string(),
            api_version: constants::VERSION_API.to_string(),
            is_connect,
        })
    }
}

impl TraitOutgoing for AppInfoOutgoing {
    fn print(&self) {
        println!("aurora-bot v{} (api: v{})", self.version, self.api_version)
    }

    fn to_json(&self) -> String {
        DataOutgoing::serialize(AppInfoIncoming::name(), self.clone())
    }
}
