use colored::Colorize;
use dialoguer::Select;
use serde::Deserialize;

use crate::feature::ClientMethodsKey;
use crate::feature::incoming::TraitIncoming;
use crate::tools::macros::tr;

#[derive(Deserialize, Clone)]
pub struct SelectorCmdIncoming {
    variants: Vec<SelectorCmdVariantIncoming>,
}

#[derive(Deserialize, Clone)]
pub struct SelectorCmdVariantIncoming {
    key: ClientMethodsKey,
    #[serde(alias = "nameData")]
    #[serde(rename = "nameData")]
    name_data: String,
    #[serde(alias = "stringData")]
    #[serde(rename = "stringData")]
    string_data: String,
}

impl SelectorCmdIncoming {
    pub fn select(&self) -> Result<Box<dyn TraitIncoming>, Box<dyn std::error::Error>> {
        let mut items: Vec<String> = vec![];
        for (i, item) in self.variants.iter().enumerate() {
            items.push(format!("{}. {}", i + 1, item.name_data));
        }
        let index = Select::new()
            .with_prompt(tr!("Выберите вариант").blue().to_string())
            .default(0)
            .items(&items)
            .interact()
            .unwrap();
        self.variants[index].key.deserialize(&self.variants[index].string_data)
    }
}
