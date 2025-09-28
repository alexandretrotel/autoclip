# AutoClip

Automatic video editor that trims blank/silent segments and highlights the best moments.

This is an **experimental** Rust tool that:
- extracts frames and audio from a video
- detects empty frames and silent audio regions
- scores segments using a small ML model
- stitches the best segments into a concise final cut


## Status (WIP 🚧)

The project scaffolding, CLI, and processing pipeline are in place, but most heavy-lifting functions are placeholders and need implementation:
- Frame extraction (ffmpeg)
- Audio loading and silence detection
- Video analysis (empty frames)
- ML scoring
- Final editing (trim/concat)

You can build and run the CLI, but actual processing is not yet implemented. It will come soon however :)

## Requirements

AutoClip is a Rust project with native dependencies for media processing, including OpenCV.

- Rust (stable) with Cargo
- macOS, Linux, or Windows (macOS tested first)
- ffmpeg (runtime + dev libs)
- OpenCV (required by the `opencv` crate)

On macOS (Homebrew):

```bash
brew install ffmpeg opencv
```

Notes:
- The `ffmpeg-next` crate links against system FFmpeg; having Homebrew FFmpeg usually works out of the box.
- The `opencv` crate requires a system OpenCV. See [installation guide](https://github.com/twistedfall/opencv-rust/blob/master/INSTALL.md).


## Build

```bash
cargo build --release
```


## Usage

```
autoclip <INPUT> <OUTPUT> [CONFIG]

Arguments:
	<INPUT>   Input video file
	<OUTPUT>  Output video file
	[CONFIG]  Optional config file (JSON)
```

Examples:

```bash
# Show help
cargo run -- help

# Basic run
cargo run -- input.mp4 output.mp4

# With config file
cargo run -- input.mp4 output.mp4 config.json
```

Enable logs:

```bash
RUST_LOG=info cargo run -- input.mp4 output.mp4
```


## Configuration

Config is a JSON file with the following fields:

```json
{
	"frame_rate": 5,
	"motion_threshold": 0.02,
	"silence_threshold": 0.01
}
```

Defaults (used when no config is provided):
- `frame_rate`: 5 (frames per second)
- `motion_threshold`: 0.02
- `silence_threshold`: 0.01

Meaning:
- `frame_rate`: target FPS for frame extraction/analysis.
- `motion_threshold`: threshold to decide “empty/static” frames.
- `silence_threshold`: amplitude threshold to mark audio segments as silent.


## Pipeline Overview

High-level steps:
1. Load config.
2. Extract frames.
3. Load audio.
4. Detect empty frames.
5. Detect silence.
6. Score clips (using ML inference).
7. Create final video.


## Development Notes

Relevant crates (from `Cargo.toml`):
- `ffmpeg-next` – media IO and processing
- `opencv` – optional image processing / analysis helpers
- `hound` – WAV IO utilities (helpful for audio debugging)
- `clap`, `log`, `env_logger`, `anyhow`, `serde`, `serde_json`


## Troubleshooting

- OpenCV not found at build time
	- Ensure OpenCV is installed. On macOS with Apple Silicon:

- FFmpeg link errors
	- Ensure FFmpeg is installed and headers/libs are visible to the linker. On macOS with Homebrew this is typically automatic.


## License

MIT
