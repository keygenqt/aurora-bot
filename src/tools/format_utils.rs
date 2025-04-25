use std::fs::File;
use std::fs::{self};
use std::io::Write;
use std::path::PathBuf;

use clang_format::ClangFormatStyle;
use clang_format::clang_format_with_style;
use walkdir::WalkDir;

use crate::service::command::exec;

use super::macros::tr;
use super::programs;
use super::utils;

pub struct FormatResult {
    pub count_files: usize,
    pub count_exclude: usize,
    pub count_formats: usize,
}

/// Format Dart project
pub fn dart_format(path: &PathBuf, dart: &PathBuf) -> Result<FormatResult, Box<dyn std::error::Error>> {
    // Check folder is dir
    if !path.is_dir() {
        Err(tr!("укажите директорию проекта"))?
    }
    if !dart.exists() {
        Err(tr!("укажите путь к Dart"))?
    }
    // Search files for format
    let files = search_files(path, [".dart"]);

    // Format
    let output = exec::exec_wait_args(
        &dart.to_string_lossy(),
        ["format", "--line-length=120", &path.to_string_lossy()],
    )?;
    let lines = utils::parse_output(output.stdout);
    Ok(FormatResult {
        count_files: files.len(),
        count_exclude: 0,
        count_formats: lines.len() - 1,
    })
}

/// Format C++ project
pub fn cpp_format(path: &PathBuf) -> Result<FormatResult, Box<dyn std::error::Error>> {
    // Check exist dependency
    let _ = programs::get_clang_format()?;
    // Check folder is dir
    if !path.is_dir() {
        Err(tr!("укажите директорию проекта"))?
    }
    // Search files for format
    let files = search_files(path, [".h", ".hpp", ".cpp"]);
    // Format
    let mut count_exclude = 0;
    let mut count_formats = 0;
    for file_path in &files {
        // Filter 3rdparty
        if file_path.to_string_lossy().contains("3rdparty") {
            count_exclude += 1;
            continue;
        }
        // Get content
        let content = fs::read_to_string(&file_path)?;
        // Format
        let output = if path.join(".clang-format").exists() {
            let format_path = path.join(".clang-format");
            let format_path = format_path.to_string_lossy();
            clang_format_with_style(&content, &ClangFormatStyle::Custom(format!("file:{}", format_path)))?
        } else {
            clang_format_with_style(
                &content,
                &ClangFormatStyle::Custom("{ BasedOnStyle: Chromium, ReflowComments: false, ColumnLimit: 120 }".to_string()),
            )?
        };
        // Save format to file
        if content != output {
            let mut file = File::create(&file_path)?;
            file.write_all(output.as_bytes())?;
            count_formats += 1;
        }
    }
    Ok(FormatResult {
        count_files: files.len(),
        count_exclude,
        count_formats,
    })
}

/// Search files
fn search_files<'s>(path: &PathBuf, extensions: impl IntoIterator<Item = &'s str>) -> Vec<PathBuf> {
    let extensions = extensions.into_iter().map(|e| e.to_string()).collect::<Vec<String>>();
    let mut result: Vec<PathBuf> = vec![];
    for entry in WalkDir::new(path)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        for extension in &extensions {
            if entry.path().to_string_lossy().ends_with(extension) {
                result.push(entry.into_path());
                break;
            }
        }
    }
    result
}
