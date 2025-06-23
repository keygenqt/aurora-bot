use colored::Colorize;
use nipper::Document;
use regex::Regex;

/// Format to markdown
pub fn format_html_to_md(body: &String) -> String {
    let clear_symbols = body.replace("`", "	&apos;");
    let clear_links = format_links(&clear_symbols);
    let clear_md = html_nipper_md(&clear_links);
    clear_md
        .replace("\n⌫\n", "\n")
        .replace(">⌫\n", ">")
        .replace("⌫\n", "")
        .trim()
        .into()
}

fn html_nipper_md(body: &String) -> String {
    let document = Document::from(body.as_str());
    document.select("b").iter().for_each(|mut e| {
        e.replace_with_html(format!("**{}**", e.text()));
    });
    document.select("i").iter().for_each(|mut e| {
        e.replace_with_html(format!("_{}_", e.text()));
    });
    document.select("u").iter().for_each(|mut e| {
        e.replace_with_html(format!("_u_{}_/u_", e.text()));
    });
    document.select("s").iter().for_each(|mut e| {
        e.replace_with_html(format!("~~{}~~", e.text()));
    });
    document.select("span").iter().for_each(|mut e| {
        e.replace_with_html(format!("~~{}~~", e.text()));
    });
    document.select("pre").select("code").iter().for_each(|e| {
        if !e.attr("class").is_none() {
            let lang = match e.attr("class").unwrap().to_string().as_str() {
                "language-py" => "python",
                "language-php" => "php",
                "language-cpp" => "cpp",
                _ => "shell",
            };
            e.parent()
                .replace_with_html(format!("```{}\n{}\n```", lang, e.text().to_string().trim()));
        }
    });
    document.select("code").iter().for_each(|mut e| {
        e.replace_with_html(format!("`{}`", e.text()));
    });
    document.select("pre").iter().for_each(|mut e| {
        e.replace_with_html(format!("`{}`", e.text()));
    });
    document.select("blockquote").iter().for_each(|mut e| {
        let value: String = e
            .text()
            .to_string()
            .replace("⌫", "")
            .trim()
            .split("\n")
            .map(|e| format!("> {}\n", e))
            .collect();
        e.replace_with_html(value);
    });
    document.select("a").iter().for_each(|mut e| {
        e.replace_with_html(format!("[{}]({})", e.text(), e.attr("href").unwrap()));
    });
    document
        .select("body")
        .text()
        .replace("_u_", "<u>")
        .replace("_/u_", "</u>")
        .trim()
        .to_string()
}

/// Format to colorize terminal
pub fn format_html_to_terminal(body: &String) -> String {
    let clear_html = html_nipper_colorize(body);
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

// @todo check
fn html_nipper_colorize(body: &String) -> String {
    let document = Document::from(body.as_str());
    document.select("b").iter().for_each(|mut e| {
        e.replace_with_html(e.text());
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

fn format_links(body: &String) -> String {
    let re = Regex::new(r####"([^"])(http[s]?.+)(\n|\s)"####).unwrap();
    let result = re.replace(body, r####"$1<a href="$2">$2</a>$3"####);
    return result.to_string();
}
