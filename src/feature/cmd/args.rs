use colored::Colorize;

/// Handling interface events
pub async fn run(command: Option<Vec<String>>) {
    // @todo add ai cli
    match command {
        Some(command) => println!("{:?}", command.join(" ")),
        None => println!(
            "{}: enter the command in free form",
            "error".bright_red().bold()
        ),
    }
}
