use crate::config::Config;

pub fn extract_frames(_video_path: &str, _cfg: &Config) -> anyhow::Result<Vec<Vec<u8>>> {
    // TODO: Use ffmpeg-next to extract frames
    Ok(vec![])
}
