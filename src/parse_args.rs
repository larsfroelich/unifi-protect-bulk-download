use clap::{ArgMatches, Command};

pub fn parse_args() -> ArgMatches {
    let cmd = Command::new("cargo")
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
                ),
        );

    cmd.get_matches()
}
