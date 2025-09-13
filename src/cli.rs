use clap::Parser;

#[derive(Parser)]
#[command(author, version, about = "AutoClip", long_about = None)]
pub struct Cli {
    /// Input video file
    pub input: String,

    /// Output video file
    pub output: String,

    /// Optional config file
    pub config: Option<String>,
}
