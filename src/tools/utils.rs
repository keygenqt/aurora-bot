use base64::Engine;
use base64::engine::general_purpose;
use cached::proc_macro::once;
use colored::Colorize;
use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use url::Url;
use walkdir::DirEntry;
use walkdir::WalkDir;

use crate::service::responses::gitlab_tags::GitlabTagsResponse;
use crate::tools::macros::crash;
use crate::tools::single;

use super::constants;
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
        None => Err(tr!("не удалось найти ключ"))?,
    }
}

/// Search bool value by any text-config
pub fn config_get_bool(params: &Vec<String>, key: &str, find: &str) -> Result<bool, Box<dyn std::error::Error>> {
    match params.iter().filter(|e| e.contains(key)).next() {
        Some(option) => Ok(option.contains(find)),
        None => Err(tr!("не удалось найти ключ"))?,
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

/// Get home folder without HOME
pub fn get_home_folder_path() -> PathBuf {
    match env::var("HOME") {
        Ok(path_home) => Path::new(&path_home).to_path_buf(),
        Err(_) => env::current_dir().unwrap_or_else(|_| crash!("директория конфигурации не найдена")),
    }
}

/// Get path for save config-file
pub fn get_file_save_path(file_name: &str) -> PathBuf {
    let path = get_home_folder_path().join(constants::CONFIGURATION_DIR);
    let _ = fs::create_dir(&path);
    path.join(file_name)
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

/// Get absolute path to file
pub fn path_to_absolute(path: &PathBuf) -> Option<PathBuf> {
    let path_str = path.to_string_lossy();
    let path = if path_str.contains("~/") {
        let path_home = format!("{}/", get_home_folder_path().to_string_lossy());
        &PathBuf::from(path_str.replace("~/", &path_home))
    } else {
        path
    };
    if !path.exists() {
        None
    } else {
        match path.canonicalize() {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }
}

/// Move files to ~Download PC
pub fn move_to_downloads(paths: Vec<PathBuf>) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut result: Vec<PathBuf> = vec![];
    let path_en = path_to_absolute(&PathBuf::from("~/Downloads"));
    let path_ru = path_to_absolute(&PathBuf::from("~/Загрузки"));
    let path_download = if path_ru.is_some() {
        path_ru.unwrap()
    } else {
        if path_en.is_none() {
            let mut create_dir = get_home_folder_path();
            create_dir.push("Downloads");
            fs::create_dir(&create_dir)?;
            create_dir
        } else {
            path_en.unwrap()
        }
    };
    for path in paths {
        let mut copy_to = path_download.clone();
        copy_to.push(path.file_name().unwrap());
        fs::rename(path, &copy_to)?;
        result.push(copy_to);
    }
    Ok(result)
}

/// Check is Url and convert API Url
pub fn get_https_url(url: String) -> Option<String> {
    let re = Regex::new(r"^/uploads*").unwrap();
    if re.captures(&url).is_some() {
        Some(format!("{}{}", constants::URL_API, url))
    } else {
        match Url::parse(&url) {
            Ok(url) => {
                if url.scheme() == "https" {
                    Some(url.to_string())
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }
}

/// Get list urls sdk
#[once(time = 300)]
pub fn get_repo_url_sdk() -> Vec<String> {
    match single::get_request().get_repo_url_files(&vec!["AuroraSDK"], None) {
        Ok(value) => value,
        Err(_) => vec![],
    }
}

/// Get list urls psdk
#[once(time = 300)]
pub fn get_repo_url_psdk() -> Vec<String> {
    match single::get_request().get_repo_url_files(&vec!["PlatformSDK", "AuroraPSDK"], None) {
        Ok(value) => value,
        Err(_) => vec![],
    }
}

/// Get list flutters
#[once(time = 300)]
pub fn get_repo_flutter() -> Vec<GitlabTagsResponse> {
    single::get_request().get_repo_tags_flutter()
}

/// Get package_name from path rpm
pub fn get_package_name(path: &PathBuf) -> Option<String> {
    let package = match rpm::Package::open(path) {
        Ok(value) => value,
        Err(_) => return None,
    };
    match package.metadata.get_name() {
        Ok(value) => Some(value.to_string()),
        Err(_) => None,
    }
}
