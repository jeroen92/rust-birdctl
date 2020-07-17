use clap::ArgMatches;

use crate::socket;

const PATH: &str = "/home/jschutrup/Projects/rust/bird/files/socket/bird.ctl";

pub fn version(cli_argument: &ArgMatches) {
    println!("BirdCTL version 0.0.1");
}

pub fn connect(cli_argument: &ArgMatches) {
    let bird_ctl = socket::connect(PATH.to_string());
}
