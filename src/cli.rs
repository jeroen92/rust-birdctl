use clap::ArgMatches;

use crate::config;
use crate::socket;
use crate::output;

pub fn version(_cli_arguments: &ArgMatches) {
    println!("BirdCTL version {}", config::VERSION);
}

pub fn connect(cli_arguments: &ArgMatches) {
    let bird_socket_path = cli_arguments.value_of("socket_path").unwrap().to_string();
    let _bird_ctl = socket::BirdSocket::new(bird_socket_path);
    println!("Connected!");
}

pub fn run_command(cli_arguments: &ArgMatches) {
    let bird_socket_path = cli_arguments.value_of("socket_path").unwrap().to_string();
    let mut bird_ctl = socket::BirdSocket::new(bird_socket_path);
    bird_ctl.send_command("show route");
    let raw_output = bird_ctl.read_output();
    let parsed_output = output::parse(raw_output);
    println!("{}", parsed_output);
}
