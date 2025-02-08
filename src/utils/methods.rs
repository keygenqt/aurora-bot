use crate::utils::macros::print_error;
use colored::Colorize;
use regex::Regex;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::{env, ffi::OsStr, process::Output};
use walkdir::{DirEntry, WalkDir};

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

pub fn config_output_filter_keys<I, S>(
    output: Output,
    keys: I,
) -> Result<Vec<String>, Box<dyn std::error::Error>>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let args: Vec<String> = keys
        .into_iter()
        .map(|k| String::from(k.as_ref().to_str().unwrap()))
        .collect();
    let params = String::from_utf8(output.stdout)?
        .split("\n")
        .map(|e| {
            if args.iter().any(|k| e.contains(k)) {
                return Some(e);
            } else {
                None
            }
        })
        .filter(|e| e.is_some())
        .map(|e| String::from(e.unwrap()))
        .collect();
    Ok(params)
}

pub fn config_vec_filter_keys<I, S>(
    output: Vec<String>,
    keys: I,
) -> Result<Vec<String>, Box<dyn std::error::Error>>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut full_vec: Vec<String> = vec![];
    let args: Vec<String> = keys
        .into_iter()
        .map(|k| String::from(k.as_ref().to_str().unwrap()))
        .collect();
    for item in output.iter() {
        let lines = item.split("\n").collect::<Vec<&str>>();
        for line in lines.iter() {
            if args.iter().any(|k| line.contains(k)) {
                full_vec.push(line.to_string());
            }
        }
    }
    Ok(full_vec)
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
        Err(_) => env::current_dir().unwrap_or_else(|_| {
            print_error!("директория конфгурации не найдена");
            exit(1)
        }),
    }
}

pub fn get_file_save(file_name: &str) -> PathBuf {
    get_home_folder().join(file_name)
}

pub fn is_file(entry: &DirEntry) -> bool {
    if let Ok(metadata) = entry.metadata() {
        metadata.is_file()
    } else {
        false
    }
}

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
            if !file_path.contains("/Trash/") && is_file(&entry) && re.is_match(entry.path().to_str().unwrap()) {
                if let Some(path_str) = entry.path().to_str() {
                    result.push(path_str.to_string());
                }
            }
        }
    }
    result
}
