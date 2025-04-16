use serde::Deserialize;
use serde::Serialize;

use crate::feature::incoming::TraitIncoming;
use crate::feature::outgoing::OutgoingType;
use crate::feature::outgoing::TraitOutgoing;

#[derive(Serialize, Deserialize, Clone)]
pub struct SelectorIncoming<T: TraitIncoming> {
    pub name: String,
    pub incoming: T,
}

impl<T: TraitIncoming> SelectorIncoming<T> {
    pub fn name() -> String {
        // Ony inner incoming
        "Selector".to_string()
    }
}

impl<T: TraitIncoming> TraitIncoming for SelectorIncoming<T> {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        self.incoming.run(send_type)
    }
}
