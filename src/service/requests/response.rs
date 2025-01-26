use color_eyre::owo_colors::OwoColorize;
use dialoguer::Select;
use serde::{Deserialize, Serialize};

use crate::utils::methods;

#[derive(Deserialize, Serialize, Debug)]
pub struct ApiResponse {
    pub code: u32,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserResponse {
    #[serde(alias = "tgId")]
    tg_id: u64,
    #[serde(alias = "firstName")]
    first_name: String,
    #[serde(alias = "lastName")]
    last_name: String,
    #[serde(alias = "userName")]
    user_name: String,
    #[serde(alias = "mode")]
    mode: String,
}

#[derive(Deserialize, Serialize, Debug)]
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

impl FaqResponse {
    pub fn print(&self) {
        println!(
            "🔖 {}\n\n{}\n\n⭐ {:.2} {} {}, {}",
            self.title.bold().cyan(),
            methods::dataset_text_clear(self.text.clone()),
            self.rating.italic().yellow(),
            self.fname.italic(),
            self.lname.italic(),
            self.date.italic(),
        )
    }
}

pub struct FaqResponses(pub Vec<FaqResponse>);

impl FaqResponses {
    pub fn print(&self) {
        if self.0.len() == 1 {
            self.0.first().unwrap().print();
        } else {
            let mut items: Vec<String> = vec![];
            for (i, item) in self.0.iter().enumerate() {
                items.push(format!("{}. {}", i + 1, item.title));
            }
            let index = Select::new()
                .with_prompt("📚 Варианты ответа".blue().to_string())
                .default(0)
                .items(&items)
                .interact()
                .unwrap();
            println!("");
            self.0[index].print();
        }
    }
}
