use std::path::PathBuf;

extern crate ffmpeg_next as ffmpeg;

// @todo ffmpeg
// 1. Resize video with crop black area
// 2. Convert to mp4
// 3. Create preview gif in base64
#[allow(dead_code)]
#[allow(unused_variables)]
pub fn ffmpeg_webm_convert(path: &PathBuf) -> Result<(PathBuf, String), Box<dyn std::error::Error>>  {
    Err("@todo")?
}
