use clap::{Subcommand, Parser, Args};

/// Tool for bulk-downloading recordings from unifi protect
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, propagate_version = true,
help_template = "{before-help}{name} {version} by {author}
{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
// https://stackoverflow.com/questions/71991935/how-to-make-a-default-subcommand-with-clap-and-derive
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Bulk-download recordings from unifi protect
    Download(DownloadCommand)
}

#[derive(Args, Debug)]
pub struct DownloadCommand {
    /// The uri of the unifi protect server host (e.g. "192.168.0.12")
    #[arg(long)]
    pub host: String,

    /// The username for logging into the unifi protect server
    #[arg(short, long)]
    pub username: String,

    /// The password for logging into the unifi protect server
    #[arg(short, long)]
    pub password: String,

    /// The path to the directory to download the files to
    #[arg(long, default_value_t = String::from("./unifi-protect-export/"))]
    pub path: String,

    /// The mode to download the files in (daily or hourly)
    #[arg(long, default_value_t = String::from("daily"),
    value_parser = clap::builder::PossibleValuesParser::new(&["daily", "hourly",])
    )]
    pub mode: String,

    /// The type of recording to download (rotating or timelapse)
    #[arg(long, default_value_t = String::from("daily"),
    value_parser = clap::builder::PossibleValuesParser::new(&["rotating", "timelapse",])
    )]
    pub recording_type: String,

    /// The start date to download the files from (YYYY-MM-DD or "now", "today", "yesterday", "2023")
    #[arg(long, default_value_t = String::from("today"))]
    pub start_date: String,

    /// The end date to download the files from (YYYY-MM-DD)
    #[arg(long, default_value_t = String::from("now"))]
    pub end_date: String,
}