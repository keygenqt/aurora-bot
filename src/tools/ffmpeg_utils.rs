use std::path::{Path, PathBuf};

// @todo ffmpeg
// extern crate ffmpeg_next as ffmpeg;

// 1. Resize video with crop black area
// 2. Convert to mp4
// 3. Create preview gif in base64
//
// There are more priority tasks for now, but a start has been made =)
#[allow(dead_code)]
#[allow(unused_variables)]
pub fn ffmpeg_webm_convert(path: &PathBuf) -> Result<(PathBuf, String), Box<dyn std::error::Error>>  {
    let new_video_file_path = Path::new("@todo").to_path_buf();
    let base64_gif = "@todo".to_string();
    Ok((new_video_file_path, base64_gif))
}
