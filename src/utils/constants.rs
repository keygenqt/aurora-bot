use clap::builder::styling::Styles;
use colored::Colorize;

/**
 * See also `clap_cargo::style::CLAP_STYLING`
 */
pub fn clap_style() -> Styles {
    clap::builder::styling::Styles::styled()
        .header(clap_cargo::style::HEADER)
        .usage(clap_cargo::style::USAGE)
        .literal(clap_cargo::style::LITERAL)
        .placeholder(clap_cargo::style::PLACEHOLDER)
        .error(clap_cargo::style::ERROR)
        .valid(clap_cargo::style::VALID)
        .invalid(clap_cargo::style::INVALID)
}

/**
 * Main help app
 */
pub fn app_about() -> String {
    format!(
        r#"

{} | {} - приложение упрощающие работу с инфраструктурой ОС Аврора.

{}"#,
        "Aurora Bot".green(),
        "Client".blue(),
        "Это сторонний инструмент, написанный энтузиастами!".italic()
    )
}
