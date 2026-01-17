// parse_args.rs - CLI argument parsing for Unifi Protect Bulk Download
// Uses clap for a clean, type-safe argument interface.

use clap::{ArgMatches, Command};

/// Parse command-line arguments and return the matches
///
/// Subcommands:
/// - download: Download footage from Unifi Protect NVR
pub fn parse_args() -> ArgMatches {
    let cmd = Command::new("unifi-protect-bulk-download")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Lars Frölich, Hue & Aye")
        .about("Bulk download footage from Unifi Protect NVR")
        .subcommand(
            Command::new("download")
                .about("Download footage from cameras")
                .arg(
                    clap::Arg::new("uri")
                        .value_name("URI")
                        .help("The URI of the Unifi Protect server (e.g., https://192.168.1.1 or https://protect.example.com)")
                        .required(true),
                )
                .arg(
                    clap::Arg::new("username")
                        .value_name("USERNAME")
                        .help("Username for Unifi Protect login")
                        .required(true),
                )
                .arg(
                    clap::Arg::new("password")
                        .value_name("PASSWORD")
                        .help("Password for Unifi Protect login")
                        .required(true),
                )
                .arg(
                    clap::Arg::new("out_path")
                        .value_name("PATH")
                        .help("Directory to save downloaded videos")
                        .required(true),
                )
                .arg(
                    clap::Arg::new("mode")
                        .value_name("MODE")
                        .value_parser(clap::builder::PossibleValuesParser::new([
                            "daily", "hourly",
                        ]))
                        .help("Download mode: 'daily' (one file per day) or 'hourly' (one file per hour)")
                        .required(true),
                )
                .arg(
                    clap::Arg::new("start_date")
                        .value_name("START_DATE")
                        .help("Start date (YYYY-MM-DD)")
                        .required(true),
                )
                .arg(
                    clap::Arg::new("end_date")
                        .value_name("END_DATE")
                        .help("End date (YYYY-MM-DD)")
                        .required(true),
                ),
        );

    cmd.get_matches()
}

#[cfg(test)]
mod tests {
    // Note: Testing clap argument parsing directly is tricky because get_matches()
    // reads from std::env::args. For unit tests, you'd typically use try_get_matches_from()
    // with a test vector. For now, we rely on integration testing via cargo run.
}
