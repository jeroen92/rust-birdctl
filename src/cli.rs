use clap::ArgMatches;

use crate::config;
use crate::socket;
use crate::prompt;

pub fn version(_cli_arguments: &ArgMatches) {
    println!("BirdCTL version {}", config::VERSION);
}

pub fn connect(cli_arguments: &ArgMatches) {
    let bird_socket_path = cli_arguments.value_of("socket_path").unwrap().to_string();
    let _bird_ctl = socket::BirdSocket::new(bird_socket_path);
    println!("Connected!");
}

pub fn prompt(cli_arguments: &ArgMatches) {
    let bird_socket_path = cli_arguments.value_of("socket_path").unwrap().to_string();
    let bird_ctl = socket::BirdSocket::new(bird_socket_path);
    let mut prompt = prompt::Prompt::new(bird_ctl);
    prompt.start();
}
