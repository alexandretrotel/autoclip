pub fn load_audio(video_path: &str) -> anyhow::Result<Vec<f32>> {
    // TODO: Extract audio waveform using hound / ffmpeg-next
    Ok(vec![])
}

pub fn detect_silence(audio: &Vec<f32>) -> anyhow::Result<Vec<(usize, usize)>> {
    // TODO: Detect silent segments
    Ok(vec![])
}
