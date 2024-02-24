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

pub fn parse_args() -> Cli {
    /*let cmd = Command::new("cargo")
        .bin_name("unifi_protect_bulk_download")
        .version("0.1.0")
        .subcommand(
            Command::new("download")
                .arg(
                    clap::Arg::new("uri")
                        .value_name("uri")
                        .value_parser(clap::value_parser!(String))
                        .help("The uri of the unifi protect server")
                        .required(true),
                )
                .arg(
                    clap::Arg::new("username")
                        .value_name("username")
                        .value_parser(clap::value_parser!(String))
                        .help("The username for logging into the unifi protect server")
                        .required(true),
                )
                .arg(
                    clap::Arg::new("password")
                        .value_name("password")
                        .value_parser(clap::value_parser!(String))
                        .help("The password for logging into the unifi protect server")
                        .required(true),
                )
                .arg(
                    clap::Arg::new("out_path")
                        .value_name("path")
                        .value_parser(clap::value_parser!(String))
                        .help("The path to the directory to download the files to")
                        .required(true),
                )
                .arg(
                    clap::Arg::new("mode")
                        .value_name("mode")
                        .value_parser(clap::builder::PossibleValuesParser::new(&[
                            "daily", "hourly",
                        ]))
                        .help("The mode to download the files in (daily or hourly)")
                        .required(true),
                )
                .arg(
                    clap::Arg::new("recording_type")
                        .value_name("recording_type")
                        .value_parser(clap::builder::PossibleValuesParser::new(&[
                            "rotating", "timelapse",
                        ]))
                        .help("The type of recording to download (rotating or timelapse)")
                        .required(true),
                )
                .arg(
                    clap::Arg::new("start_date")
                        .value_name("start_date")
                        .value_parser(clap::value_parser!(String))
                        .help("The start date to download the files from (YYYY-MM-DD)")
                        .required(true),
                )
                .arg(
                    clap::Arg::new("end_date")
                        .value_name("end_date")
                        .value_parser(clap::value_parser!(String))
                        .help("The end date to download the files from (YYYY-MM-DD)")
                        .required(true),
                )
                .arg(
                    clap::Arg::new("cameras")
                        .value_name("cameras")
                        .value_parser(clap::value_parser!(String))
                        .help("A comma-separated list of cameras if you want to download from specific cameras, or 'all'/'*' to download from all cameras")
                        .value_delimiter(',')
                        .required(true),
                ),
        );
*/
    Cli::parse()
}