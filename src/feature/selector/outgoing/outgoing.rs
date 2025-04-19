use colored::Colorize;
use dialoguer::Select;
use serde::Deserialize;
use serde::Serialize;

use crate::feature::incoming::TraitIncoming;
use crate::feature::outgoing::DataOutgoing;
use crate::feature::outgoing::OutgoingType;
use crate::feature::outgoing::TraitOutgoing;
use crate::tools::macros::tr;

use super::incoming::SelectorIncoming;

#[derive(Serialize, Deserialize, Clone)]
pub struct SelectorOutgoing<T: TraitIncoming + Serialize + Clone> {
    pub key: String,
    pub variants: Vec<SelectorIncoming<T>>,
    #[serde(skip_serializing)]
    pub send_type: OutgoingType,
}

impl<T: TraitIncoming + Serialize + Clone> TraitOutgoing for SelectorOutgoing<T> {
    fn print(&self) {
        let mut items: Vec<String> = vec![];
        for (i, item) in self.variants.iter().enumerate() {
            items.push(format!("{}. {}", i + 1, item.name));
        }
        let index = Select::new()
            .with_prompt(tr!("Выберите вариант").blue().to_string())
            .default(0)
            .items(&items)
            .interact()
            .unwrap();
        self.variants[index].incoming.run(OutgoingType::Cli).print();
    }

    fn to_json(&self) -> String {
        if self.send_type == OutgoingType::Cli {
            let mut items: Vec<String> = vec![];
            for (i, item) in self.variants.iter().enumerate() {
                items.push(format!("{}. {}", i + 1, item.name));
            }
            let index = Select::new()
                .with_prompt(tr!("Выберите вариант").blue().to_string())
                .default(0)
                .items(&items)
                .interact()
                .unwrap();
            self.variants[index].incoming.run(OutgoingType::Cli).to_json()
        } else {
            DataOutgoing::serialize(SelectorIncoming::<T>::name(), self.clone())
        }
    }
}
