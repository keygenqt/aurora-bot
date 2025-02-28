use base64::Engine;
use base64::engine::general_purpose;
use colored::Colorize;
use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use walkdir::DirEntry;
use walkdir::WalkDir;

use crate::tools::macros::crash;

use super::macros::tr;

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

/// Search string value by any text-config
pub fn config_get_string(params: &Vec<String>, key: &str, split: &str) -> Result<String, Box<dyn std::error::Error>> {
    match params.iter().filter(|e| e.contains(key)).next() {
        Some(option) => Ok(option
            .split(split)
            .skip(1)
            .collect::<String>()
            .trim()
            .trim_matches(&['"'] as &[_])
            .to_string()),
        None => Err("не удалось найти ключ")?,
    }
}

/// Search bool value by any text-config
pub fn config_get_bool(params: &Vec<String>, key: &str, find: &str) -> Result<bool, Box<dyn std::error::Error>> {
    match params.iter().filter(|e| e.contains(key)).next() {
        Some(option) => Ok(option.contains(find)),
        None => Err("не удалось найти ключ")?,
    }
}

/// Get home folder without HOME
pub fn get_home_folder_path() -> PathBuf {
    match env::var("HOME") {
        Ok(path_home) => Path::new(&path_home).to_path_buf(),
        Err(_) => env::current_dir().unwrap_or_else(|_| crash!("директория конфигурации не найдена")),
    }
}

/// Get path for save config-file
pub fn get_file_save_path(file_name: &str) -> PathBuf {
    get_home_folder_path().join(file_name)
}

/// Gen path for screenshot
pub fn get_screenshot_save_path() -> PathBuf {
    let start = SystemTime::now();
    let timestamp = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();
    let file_name = format!("Screenshot_{}.png", timestamp.to_string());
    let en_path = get_home_folder_path().join("Pictures").join("Screenshots");
    if en_path.exists() {
        en_path.join(file_name)
    } else {
        get_home_folder_path().join(file_name)
    }
}

/// Check is file
pub fn is_file(entry: &DirEntry) -> bool {
    if let Ok(metadata) = entry.metadata() {
        metadata.is_file()
    } else {
        false
    }
}

/// Search file by PC
pub fn search_files(path: &str) -> Vec<String> {
    let reg = format!("^.*{}$", path);
    let re = Regex::new(&reg).unwrap();
    let mut result: Vec<String> = vec![];
    for entry in WalkDir::new(get_home_folder_path())
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let file_path = entry.path().to_string_lossy();
        if file_path.contains(path) {
            if !file_path.contains("/Trash/") && is_file(&entry) && re.is_match(entry.path().to_str().unwrap()) {
                if let Some(path_str) = entry.path().to_str() {
                    result.push(path_str.to_string());
                }
            }
        }
    }
    result
}

/// Output exec to list string
pub fn parse_output(out: Vec<u8>) -> Vec<String> {
    if let Ok(value) = String::from_utf8(out) {
        return value
            .split("\n")
            .filter(|e| !e.trim().is_empty())
            .map(|e| e.to_string())
            .collect();
    }
    vec![]
}

/// Get incoming object from query
pub fn clear_to_model_body(value: &String) -> Result<String, Box<dyn std::error::Error>> {
    if value.contains("jsonData") {
        let body = match value.split("jsonData").last() {
            Some(body) => body.trim(),
            None => Err(tr!("error parse"))?,
        };
        let body = match body.split("nameData").next() {
            Some(body) => body.trim(),
            None => Err(tr!("error parse"))?,
        };
        Ok(body[2..body.len() - 1].trim().trim_end_matches(',').to_string())
    } else {
        Ok(value.clone())
    }
}

// Get unique key by path for models
pub fn key_from_path(path: &String) -> String {
    let mut keys: Vec<char> = vec![];
    for block in path.split(['/', '-', '_']) {
        match block.chars().nth(0) {
            Some(c) => {
                if c == '.' {
                    keys.push(block.chars().nth(1).unwrap());
                } else if c == 'h' {
                    continue;
                } else {
                    keys.push(c);
                }
            }
            None => continue,
        }
    }
    keys.iter().collect::<String>().to_lowercase()
}

/// Get file and encode to base64
pub fn file_to_base64_by_path(path: Option<&str>) -> Option<String> {
    if path.is_some() {
        match fs::read(path.unwrap()) {
            Ok(input) => Some(general_purpose::STANDARD.encode(input)),
            Err(_) => None,
        }
    } else {
        None
    }
}

/// Change path by lambda to actual path
pub fn path_lambda_home(path: &PathBuf) -> PathBuf {
    let path_str = path.to_string_lossy();
    if path_str.contains("~/") {
        let path_home = format!("{}/", get_home_folder_path().to_string_lossy());
        let path_new = path_str.replace("~/", &path_home);
        return Path::new(&path_new).to_path_buf();
    }
    path.clone()
}

/// Get absolute path to file
pub fn path_to_absolute(path: &PathBuf) -> Option<PathBuf> {
    let path = path_lambda_home(path);
    if !path.exists() || !path.is_file() {
        None
    } else {
        match path.canonicalize() {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }
}

/// Get absolute path to file from str
pub fn path_to_absolute_str(path: &str) -> Option<PathBuf> {
    path_to_absolute(&Path::new(path).to_path_buf())
}
