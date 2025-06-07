use color_eyre::owo_colors::OwoColorize;
use dialoguer::Select;
use serde::Deserialize;
use serde::Serialize;

use crate::feature::outgoing::DataOutgoing;
use crate::feature::outgoing::TraitOutgoing;
use crate::tools::macros::tr;
use crate::tools::telegram;

#[derive(Deserialize, Serialize, Clone)]
pub struct FaqResponse {
    hash: String,
    url: String,
    title: String,
    text: String,
    fname: String,
    lname: String,
    date: String,
    timestamp: u64,
    rating: f64,
    image: Option<String>,
}

impl TraitOutgoing for FaqResponse {
    fn print(&self) {
        println!(
            "🔖 {}\n\n{}\n\n⭐ {:.2} {} {}, {}",
            self.title.bold().cyan(),
            telegram::format_html_to_terminal(&self.text),
            self.rating.italic().yellow(),
            self.fname.italic(),
            self.lname.italic(),
            self.date.italic(),
        )
    }

    fn to_json(&self) -> String {
        DataOutgoing::serialize("FaqResponse".to_string(), self.clone())
    }
}

#[derive(Serialize, Clone)]
pub struct FaqResponses(pub Vec<FaqResponse>);

impl FaqResponse {
    pub fn format_body_to_md(&self) -> FaqResponse {
        let mut model = self.clone();
        model.text = telegram::format_html_to_md(&model.text);
        model
    }
}

impl TraitOutgoing for FaqResponses {
    fn print(&self) {
        if self.0.len() == 1 {
            self.0.first().unwrap().print();
        } else {
            let mut items: Vec<String> = vec![];
            for (i, item) in self.0.iter().enumerate() {
                items.push(format!("{}. {}", i + 1, item.title));
            }
            let index = Select::new()
                .with_prompt(tr!("Выберите вариант").blue().to_string())
                .default(0)
                .items(&items)
                .interact()
                .unwrap();
            println!();
            self.0[index].print();
        }
    }

    fn to_json(&self) -> String {
        DataOutgoing::serialize("FaqResponses".to_string(), self.clone())
    }
}
