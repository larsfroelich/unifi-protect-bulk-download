use clap::{Parser, Subcommand, ValueEnum};

/// Tool for bulk-downloading recordings from unifi protect.
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = None,
    propagate_version = true,
    help_template = "{before-help}{name} {version} by {author}\n{about-with-newline}\n{usage-heading} {usage}\n\n{all-args}{after-help}"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Download footage from the UniFi Protect server.
    Download(DownloadArgs),
}

#[derive(clap::Args, Debug)]
pub struct DownloadArgs {
    /// The URI of the UniFi Protect server.
    pub uri: String,
    /// The username for logging into the UniFi Protect server.
    pub username: String,
    /// The password for logging into the UniFi Protect server.
    pub password: String,
    /// The path to the directory to download files to.
    pub out_path: String,
    /// The mode to download files in.
    pub mode: DownloadMode,
    /// The type of recording to download.
    pub recording_type: RecordingType,
    /// The start date/time to download files from (YYYY-MM-DD or YYYY-MM-DD-HH).
    pub start_date: String,
    /// The end date/time to download files to (YYYY-MM-DD or YYYY-MM-DD-HH).
    pub end_date: String,
    /// Comma-separated list of camera names/ids, or `all` / `*`.
    #[arg(value_delimiter = ',')]
    pub cameras: Vec<String>,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum DownloadMode {
    Daily,
    Hourly,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum RecordingType {
    Rotating,
    Timelapse,
}

impl RecordingType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Rotating => "rotating",
            Self::Timelapse => "timelapse",
        }
    }
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
