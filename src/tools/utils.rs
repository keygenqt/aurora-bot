use colored::Colorize;
use regex::Regex;
use std::env;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

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

pub fn config_get_string(
    params: &Vec<String>,
    key: &str,
    split: &str,
) -> Result<String, Box<dyn std::error::Error>> {
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

pub fn config_get_bool(
    params: &Vec<String>,
    key: &str,
    find: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    match params.iter().filter(|e| e.contains(key)).next() {
        Some(option) => Ok(option.contains(find)),
        None => Err("не удалось найти ключ")?,
    }
}

pub fn get_home_folder() -> PathBuf {
    match env::var("HOME") {
        Ok(path_home) => Path::new(&path_home).to_path_buf(),
        Err(_) => {
            env::current_dir().unwrap_or_else(|_| crash!("директория конфигурации не найдена"))
        }
    }
}

pub fn get_file_save(file_name: &str) -> PathBuf {
    get_home_folder().join(file_name)
}

#[allow(dead_code)]
pub fn is_file(entry: &DirEntry) -> bool {
    if let Ok(metadata) = entry.metadata() {
        metadata.is_file()
    } else {
        false
    }
}

#[allow(dead_code)]
pub fn search_files(path: &str) -> Vec<String> {
    let reg = format!("^.*{}$", path);
    let re = Regex::new(&reg).unwrap();
    let mut result: Vec<String> = vec![];
    for entry in WalkDir::new(get_home_folder())
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let file_path = entry.path().to_string_lossy();
        if file_path.contains(path) {
            if !file_path.contains("/Trash/")
                && is_file(&entry)
                && re.is_match(entry.path().to_str().unwrap())
            {
                if let Some(path_str) = entry.path().to_str() {
                    result.push(path_str.to_string());
                }
            }
        }
    }
    result
}

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

pub fn clear_to_model_body(value: &String) -> Result<String, Box<dyn std::error::Error>> {
    match value.split("\"value\":").last() {
        Some(body) => Ok(body[0..body.len() - 1].trim().to_string()),
        None => Err(tr!("error parse"))?,
    }
}
