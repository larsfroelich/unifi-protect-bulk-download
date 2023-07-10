use crate::parse_args::{parse_args};

mod parse_args;

fn main() {
    println!("Hello, world!");

    let args = parse_args();

    println!("Args: {:?}!", args);
}
