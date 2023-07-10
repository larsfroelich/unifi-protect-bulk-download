use clap::{ArgMatches};

pub fn parse_args() -> ArgMatches{
    let cmd = clap::Command::new("cargo")
        .bin_name("cargo");

    cmd.get_matches()
}
