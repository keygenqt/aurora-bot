use colored::Colorize;

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
