use clap::ArgMatches;

use crate::config;
use crate::socket;

pub fn version(_cli_arguments: &ArgMatches) {
    println!("BirdCTL version {}", config::VERSION);
}

pub fn connect(cli_arguments: &ArgMatches) {
    let bird_socket_path = cli_arguments.value_of("socket_path").unwrap().to_string();
    let bird_ctl = socket::connect(bird_socket_path);
    println!("Connected!");
}

pub fn run_command(cli_arguments: &ArgMatches) {
    let bird_socket_path = cli_arguments.value_of("socket_path").unwrap().to_string();
    let mut bird_ctl = socket::connect(bird_socket_path);
    bird_ctl.send_command("show route");
}