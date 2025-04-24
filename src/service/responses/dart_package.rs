use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct DartPackageResponse {
    pub name: String,
    pub latest: DartPackageVersionResponse,
}

#[derive(Deserialize, Clone, Debug)]
pub struct DartPackageVersionResponse {
    pub pubspec: DartPackagePubspecResponse,
}

#[derive(Deserialize, Clone, Debug)]
pub struct DartPackagePubspecResponse {
    pub version: String,
    pub description: Option<String>,
    pub repository: Option<String>,
    pub flutter: Option<serde_json::Value>,
    pub dependencies: Option<serde_json::Value>,
}
