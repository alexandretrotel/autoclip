mod audio;
mod cli;
mod config;
mod ml;
mod video;

use clap::Parser;
use cli::Cli;
use log::info;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = Cli::parse();

    info!("Starting video processing for input: {}", args.input);

    // Load configuration
    let _cfg = config::load_config(&args.config)?;

    // Extract frames & audio
    let frames = video::extractor::extract_frames(&args.input)?;
    let audio = audio::analyzer::load_audio(&args.input)?;

    // Detect empty/silent parts
    let _empty_frames = video::analyzer::detect_empty_frames(&frames)?;
    let _silent_parts = audio::analyzer::detect_silence(&audio)?;

    // Score clips using ML
    let scored_clips = ml::inference::score_clips(&frames, &audio)?;

    // Generate final video
    video::editor::create_final_video(&args.input, &scored_clips)?;

    println!("Video processing completed!");
    Ok(())
}
