use colored::Colorize;
use nipper::Document;
use regex::Regex;

/// Main help app
pub fn app_about() -> String {
    format!(
        r#"

{} | {} - приложение упрощающие работу с инфраструктурой ОС Аврора.

{}"#,
        "Aurora Bot".bright_green().bold(),
        "Client".blue(),
        "Это сторонний инструмент, написанный энтузиастами!".italic()
    )
}

pub fn dataset_text_clear(text: String) -> String {
    let clear_html = html_nipper(text);
    let clear_lines = Regex::new(r"[\n]{3,}").unwrap()
        .replace_all(&clear_html, "\n")
        .to_string();
    clear_lines
        .replace("\n⌫\n", "\n")
        .replace(">⌫\n", ">")
        .replace("⌫\n", " ")
        .trim()
        .into()
}

pub fn html_nipper(html: String) -> String {
    let document = Document::from(html.as_str());
    // @todo add other tags html telegram
    document.select("a").iter().for_each(|mut link| {
        link.replace_with_html(format!("{}: {}", link.text().blue().bold(), link.attr("href").unwrap()));
    });
    document.select("body").text().trim().to_string()
}
