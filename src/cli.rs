use clap::ArgMatches;

use crate::config;
use crate::socket;

pub fn version(cli_argument: &ArgMatches) {
    println!("BirdCTL version {}", config::VERSION);
}

pub fn connect(cli_argument: &ArgMatches) {
    let bird_ctl = socket::connect(config::SOCKET_PATH.to_string());
}
