use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub frame_rate: u32,
    pub motion_threshold: f32,
    pub silence_threshold: f32,
}

pub fn load_config(path: &Option<String>) -> anyhow::Result<Config> {
    if let Some(path) = path {
        let content = fs::read_to_string(path)?;
        let cfg: Config = serde_json::from_str(&content)?;
        Ok(cfg)
    } else {
        Ok(Config {
            frame_rate: 5,           // frames per second
            motion_threshold: 0.02,  // motion detection threshold
            silence_threshold: 0.01, // silence detection threshold
        })
    }
}
