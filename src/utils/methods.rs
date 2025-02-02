use colored::Colorize;
use nipper::Document;
use regex::Regex;

use crate::{
    app::api::{
        enums::{ClientState, SendType},
        outgoing::Outgoing,
    },
    service::{dbus::server::ServerDbus, websocket::client::ClientWebsocket},
};

use super::macros::{print_error, print_info, print_success};

/// Main help app
pub fn app_about() -> String {
    format!(
        r#"

{} - приложение упрощающие работу с инфраструктурой ОС Аврора.

{}"#,
        "Aurora Bot".bright_green().bold(),
        "Это сторонний инструмент, написанный энтузиастами!".italic()
    )
}

pub fn dataset_text_clear(text: String) -> String {
    let clear_html = html_nipper(text);
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

pub fn html_nipper(html: String) -> String {
    let document = Document::from(html.as_str());
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
            e.replace_with_html(format!(
                "❝{}❞",
                e.text().to_string().replace("⌫", "").trim()
            ));
        } else {
            e.replace_with_html(format!(
                "❝\n{}\n❞",
                e.text().to_string().replace("⌫", "").trim()
            ));
        }
    });
    document.select("a").iter().for_each(|mut e| {
        e.replace_with_html(format!(
            "{}: {}",
            e.text().blue().bold(),
            e.attr("href").unwrap()
        ));
    });
    document.select("body").text().trim().to_string()
}

pub fn send_state(outgoing: &Outgoing, send_type: &SendType) {
    match send_type {
        SendType::Cli => print_outgoing(outgoing),
        SendType::Dbus => ServerDbus::send(&outgoing),
        SendType::Websocket => ClientWebsocket::send(&outgoing),
    }
}

pub fn print_outgoing(outgoing: &Outgoing) {
    match outgoing {
        // Common
        Outgoing::Error(outgoing) => {
            let message = &outgoing.message;
            print_error!(message)
        }
        Outgoing::AppInfo(outgoing) => {
            println!("aurora-bot {}", outgoing.version)
        }
        Outgoing::EmulatorStart(outgoing) => match outgoing.state {
            ClientState::Error => print_error!("не удалось запустить эмулятор"),
            ClientState::Info => {
                let message = format!("эмулятор уже запущен: {}", outgoing.message);
                print_info!(message)
            },
            ClientState::Success => {
                let message = format!("эмулятор успешно запущен: {}", outgoing.message);
                print_success!(message)
            },
        },
        Outgoing::EmulatorStartState(outgoing) => match outgoing.code {
            1 => print_info!("поиск эмулятора..."),
            2 => print_info!("запуск эмулятора..."),
            3 => print_info!("ожидаем подключение..."),
            _ => {}
        },
        // Websocket
        Outgoing::Connection(outgoing) => {
            println!("{}", outgoing.message.green().bold())
        }
        // D-Bus
        Outgoing::ApiInfo(outgoing) => {
            println!("dbus-api {}", outgoing.version)
        }
    }
}
