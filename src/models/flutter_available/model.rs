use std::collections::HashMap;

use crate::models::TraitModel;
use crate::service::responses::gitlab_tags::GitlabTagsResponse;
use crate::tools::macros::tr;
use crate::tools::single;
use chrono::DateTime;
use colored::Colorize;
use human_sort::sort;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone)]
pub struct FlutterAvailableModel {
    pub tag: String,
    pub version: String,
    pub created_at: String,
    pub url_gitlab: String,
    pub url_zip: String,
    pub url_tar_gz: String,
}

impl TraitModel for FlutterAvailableModel {
    fn get_id(&self) -> String {
        format!("{:x}", md5::compute(self.version.as_bytes()))
    }

    fn get_key(&self) -> String {
        self.version.clone()
    }

    fn print(&self) {
        let mut data: Vec<String> = vec![];
        let created_at = match DateTime::parse_from_rfc3339(&self.created_at) {
            Ok(value) => value.format("%Y-%m-%d").to_string(),
            Err(_) => self.created_at.clone(),
        };
        let message = tr!(
            "Flutter SDK: {}\nДата релиза: {}\nGitLab: {}\nСсылка (zip): {}\nСсылка (tar.gz): {}",
            self.version.bold().white(),
            created_at.bold().white(),
            self.url_gitlab.to_string().bright_blue(),
            self.url_zip.to_string().bright_blue(),
            self.url_tar_gz.to_string().bright_blue(),
        );
        data.push(message);
        println!("{}", data.join("\n\n"));
    }
}

impl FlutterAvailableModel {
    pub fn search() -> Vec<FlutterAvailableModel> {
        match Self::search_full() {
            Ok(value) => value,
            Err(_) => vec![],
        }
    }

    pub fn search_filter<T: Fn(&FlutterAvailableModel) -> bool>(filter: T) -> Vec<FlutterAvailableModel> {
        Self::search().iter().filter(|e| filter(e)).cloned().collect()
    }

    fn search_full() -> Result<Vec<FlutterAvailableModel>, Box<dyn std::error::Error>> {
        let tags_flutter = single::get_request().get_repo_tags_flutter();
        // Clear tags version
        let mut versions: Vec<String> = vec![];
        let mut version_tags: HashMap<String, GitlabTagsResponse> = HashMap::new();
        for tag in tags_flutter {
            let version = tag.name.replace("aurora", "").trim_matches('-').to_string();
            if !version_tags.contains_key(&version) {
                version_tags.insert(version.clone(), tag);
                versions.push(version);
            }
        }
        // Sort version
        let mut versions = versions.iter().map(|e| e.as_str()).collect::<Vec<&str>>();
        sort(&mut versions);
        let reverse = versions.iter().copied().rev().collect::<Vec<&str>>();
        // Map to model
        let mut models: Vec<FlutterAvailableModel> = vec![];
        for version in reverse {
            let model = version_tags.get(version).unwrap();
            let created_at = match model.created_at.clone() {
                Some(value) => value,
                None => model.commit.committed_date.clone(),
            };
            models.push(FlutterAvailableModel {
                tag: model.name.clone(),
                version: version.to_string(),
                created_at,
                url_gitlab: format!("https://gitlab.com/omprussia/flutter/flutter/-/tree/{version}"),
                url_zip: format!(
                    "https://gitlab.com/omprussia/flutter/flutter/-/archive/{version}/flutter-{version}.zip"
                ),
                url_tar_gz: format!(
                    "https://gitlab.com/omprussia/flutter/flutter/-/archive/{version}/flutter-{version}.tar.gz"
                ),
            });
        }
        Ok(models)
    }
}
