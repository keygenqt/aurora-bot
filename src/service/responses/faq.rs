use color_eyre::owo_colors::OwoColorize;
use dialoguer::Select;
use nipper::Document;
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;

use crate::feature::outgoing::DataOutgoing;
use crate::feature::outgoing::TraitOutgoing;
use crate::tools::macros::tr;

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
            self.dataset_text_clear(),
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

impl FaqResponse {
    fn dataset_text_clear(&self) -> String {
        let clear_html = self.html_nipper();
        let clear_lines = Regex::new(r"[\n]{3,}")
            .unwrap()
            .replace_all(&clear_html, "\n")
            .to_string();
        clear_lines
            .replace("\n⌫\n", "\n")
            .replace(">⌫\n", ">")
            .replace("⌫\n", "")
            .trim()
            .into()
    }

    fn html_nipper(&self) -> String {
        let document = Document::from(self.text.as_str());
        document.select("b").iter().for_each(|mut e| {
            e.replace_with_html(e.text().bold().to_string());
        });
        document.select("i").iter().for_each(|mut e| {
            e.replace_with_html(e.text().italic().to_string());
        });
        document.select("u").iter().for_each(|mut e| {
            e.replace_with_html(e.text().underline().to_string());
        });
        document.select("s").iter().for_each(|mut e| {
            e.replace_with_html(e.text().strikethrough().to_string());
        });
        document.select("span").iter().for_each(|mut e| {
            e.replace_with_html(e.text().dimmed().to_string());
        });
        document.select("pre").select("code").iter().for_each(|e| {
            if !e.attr("class").is_none() {
                let lang = match e.attr("class").unwrap().to_string().as_str() {
                    "language-py" => "Python",
                    "language-php" => "PHP",
                    "language-cpp" => "C++",
                    "language-shell" => "Shell",
                    "language-bash" => "Bash",
                    _ => "Code",
                };
                e.parent().replace_with_html(format!(
                    "------------ {}\n{}\n------------",
                    lang,
                    e.text().to_string().trim()
                ));
            }
        });
        document.select("code").iter().for_each(|mut e| {
            e.replace_with_html(format!(" {} ", e.text()).on_bright_black().to_string());
        });
        document.select("pre").iter().for_each(|mut e| {
            e.replace_with_html(format!(" {} ", e.text()).on_bright_black().to_string());
        });
        document.select("blockquote").iter().for_each(|mut e| {
            if e.attr("expandable").is_none() {
                e.replace_with_html(format!("❝{}❞", e.text().to_string().replace("⌫", "").trim()));
            } else {
                e.replace_with_html(format!("❝\n{}\n❞", e.text().to_string().replace("⌫", "").trim()));
            }
        });
        document.select("a").iter().for_each(|mut e| {
            e.replace_with_html(format!("{}: {}", e.text().blue().bold(), e.attr("href").unwrap()));
        });
        document.select("body").text().trim().to_string()
    }
}

#[derive(Serialize, Clone)]
pub struct FaqResponses(pub Vec<FaqResponse>);

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
