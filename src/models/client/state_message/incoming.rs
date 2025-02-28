use serde::Deserialize;
use serde::Serialize;

use crate::models::client::ClientMethodsKey;
use crate::models::client::ClientMethodsState;
use crate::models::client::incoming::TraitIncoming;
use crate::models::client::outgoing::OutgoingType;
use crate::models::client::outgoing::TraitOutgoing;

use super::outgoing::StateMessageOutgoing;

#[derive(Serialize, Deserialize, Clone)]
pub struct StateMessageIncoming {
    state: ClientMethodsState,
    message: String,
}

impl StateMessageIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::StateMessage)
            .unwrap()
            .to_string()
    }
}

impl TraitIncoming for StateMessageIncoming {
    fn run(&self, _: OutgoingType) -> Box<dyn TraitOutgoing> {
        let message = self.message.clone();
        match self.state {
            ClientMethodsState::Error => StateMessageOutgoing::new_error(message),
            ClientMethodsState::Info => StateMessageOutgoing::new_info(message),
            ClientMethodsState::State => StateMessageOutgoing::new_state(message),
            ClientMethodsState::Success => StateMessageOutgoing::new_success(message),
            ClientMethodsState::Warning => StateMessageOutgoing::new_warning(message),
            ClientMethodsState::Progress => StateMessageOutgoing::new_progress(message),
        }
    }
}
