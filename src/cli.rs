use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Input video file
    pub input: String,

    /// Output video file
    pub output: String,

    /// Optional config file
    pub config: Option<String>,
}
