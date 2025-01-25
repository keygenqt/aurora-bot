use color_eyre::owo_colors::OwoColorize;
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
