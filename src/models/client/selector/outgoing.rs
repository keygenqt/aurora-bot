use colored::Colorize;
use dialoguer::Select;
use serde::Deserialize;
use serde::Serialize;

use crate::models::client::incoming::TraitIncoming;
use crate::models::client::outgoing::DataOutgoing;
use crate::models::client::outgoing::OutgoingType;
use crate::models::client::outgoing::TraitOutgoing;

use super::incoming::SelectorIncoming;

#[derive(Serialize, Deserialize, Clone)]
pub struct SelectorOutgoing<T: TraitIncoming + Serialize + Clone> {
    pub key: String,
    pub variants: Vec<SelectorIncoming<T>>,
}

impl<T: TraitIncoming + Serialize + Clone> TraitOutgoing for SelectorOutgoing<T> {
    fn print(&self) {
        let mut items: Vec<String> = vec![];
        for (i, item) in self.variants.iter().enumerate() {
            items.push(format!("{}. {}", i + 1, item.name));
        }
        let index = Select::new()
            .with_prompt("Выберите вариант".blue().to_string())
            .default(0)
            .items(&items)
            .interact()
            .unwrap();
        self.variants[index].incoming.run(OutgoingType::Cli).print();
    }

    fn to_json(&self) -> String {
        DataOutgoing::serialize(SelectorIncoming::<T>::name(), self.clone())
    }
}
