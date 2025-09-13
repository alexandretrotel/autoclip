use crate::config::Config;

pub fn detect_empty_frames(_frames: &Vec<Vec<u8>>, _cfg: &Config) -> anyhow::Result<Vec<usize>> {
    // TODO: Compute frame differences, mark empty frames
    Ok(vec![])
}
