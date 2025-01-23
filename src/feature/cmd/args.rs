use colored::Colorize;

/// Run module command
pub async fn run(command: Option<Vec<String>>) {
    match command {
        Some(command) => println!("{:?}", command.join(" ")),
        None => println!(
            "{}: enter the command in free form",
            "error".bright_red().bold()
        ),
    }
}
