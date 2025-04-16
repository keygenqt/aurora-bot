use std::collections::HashMap;
use std::fs;

use crate::feature::outgoing::TraitOutgoing;
use crate::feature::state_message::outgoing::StateMessageOutgoing;
use crate::service::command::exec;

use super::constants;
use super::macros::tr;
use super::programs;
use super::utils;

pub fn command_aliases(aliases: HashMap<&str, String>) -> Result<String, Box<dyn std::error::Error>> {
    // Map aliases
    let list_aliases: Vec<String> = aliases.iter().map(|e| format!("alias {}={}", e.0, e.1)).collect();
    // Map template
    let list_keys: Vec<String> = aliases.iter().map(|e| format!("\x1b[94m{}\x1b[0m", e.0)).collect();
    // Create file rcfile
    let file_rcfile = format!(
        r#"# Environment for run instance bash

{aliases}
printf "You can use {keys} aliases in this environment.\n\n{aliases}\n\n"

"#,
        aliases = list_aliases.join("\n"),
        keys = list_keys.join(" & ")
    );
    // Save to file
    let rcfile_path = utils::get_file_save_path(constants::ENVIRONMENT_FILE);
    fs::write(&rcfile_path, file_rcfile)?;
    let rcfile_str = rcfile_path.to_string_lossy();
    Ok(format!("bash --rcfile {}", rcfile_str))
}

pub fn open(command: String) -> Box<dyn TraitOutgoing> {
    // Try run in terminal Kitty
    if let Ok(program) = programs::get_kitty_terminal() {
        let _ = exec::exec_wait_args(&program, ["--detach", "bash", "-c", &command]);
        return StateMessageOutgoing::new_success(tr!("терминал Kitty открыт"));
    }
    // Try run in terminal Gnome
    if let Ok(program) = programs::get_gnome_terminal() {
        let _ = exec::exec_wait_args(&program, ["--", "bash", "-c", &command]);
        return StateMessageOutgoing::new_success(tr!("терминал Gnome открыт"));
    }
    StateMessageOutgoing::new_error(tr!("не удалось открыть терминал"))
}
