use clap::ArgMatches;

use crate::config;
use crate::socket;

pub fn version(cli_arguments: &ArgMatches) {
    println!("BirdCTL version {}", config::VERSION);
}

pub fn connect(cli_arguments: &ArgMatches) {
    let bird_socket_path = cli_arguments.value_of("socket_path").unwrap().to_string();
    let bird_ctl = socket::connect(bird_socket_path);
    println!("Connected!");
}
