use crate::config::Config;

pub fn load_audio(_video_path: &str, _cfg: &Config) -> anyhow::Result<Vec<f32>> {
    // TODO: Extract audio waveform using hound / ffmpeg-next
    Ok(vec![])
}

pub fn detect_silence(_audio: &Vec<f32>, _cfg: &Config) -> anyhow::Result<Vec<(usize, usize)>> {
    // TODO: Detect silent segments
    Ok(vec![])
}
