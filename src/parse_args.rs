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
                        .help("The uri of the unifi protect server")
                        .required(true),
                )
                .arg(
                    clap::Arg::new("out_path")
                        .value_name("path")
                        .help("The path to the directory to download the files to")
                        .required(true),
                )
                .arg(
                    clap::Arg::new("config")
                        .short('c')
                        .long("config")
                        .value_name("FILE")
                        .help("Sets a custom config file"),
                ),
        );

    cmd.get_matches()
}
