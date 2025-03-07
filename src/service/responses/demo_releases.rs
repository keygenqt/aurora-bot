use serde::Deserialize;

use crate::tools::enums::PlatformArch;

#[derive(Deserialize, Clone)]
pub struct DemoReleasesResponse {
    pub tag_name: String,
    assets: Vec<DemoReleasesAssetResponse>,
    pub info: Option<DemoAppResponse>,
}

impl DemoReleasesResponse {
    pub fn get_asset_platform(&self, platform: PlatformArch) -> Option<DemoReleasesAssetResponse> {
        for asset in &self.assets {
            if platform == PlatformArch::Arm32 && asset.browser_download_url.contains("armv7hl") {
                return Some(asset.clone());
            }
            if platform == PlatformArch::Arm64 && asset.browser_download_url.contains("aarch64") {
                return Some(asset.clone());
            }
            if platform == PlatformArch::Amd64 && asset.browser_download_url.contains("x86_64") {
                return Some(asset.clone());
            }
        }
        None
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct DemoReleasesAssetResponse {
    pub browser_download_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DemoAppResponse {
    pub name: String,
    pub icon: String,
    pub desc_ru: String,
    pub repo: String,
}
