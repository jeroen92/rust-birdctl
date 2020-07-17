use clap::{App, Arg, SubCommand};

mod cli;
mod config;
mod socket;

fn main() {
    let matches = App::new("BirdCTL")
        .version(config::VERSION)
        .author("Jeroen S. <jeroenschutrup@hotmail.nl")
        .about("CLI to interact with Bird routing daemon")
        .subcommand(
            SubCommand::with_name("connect").about("Test the connection to the Bird socket"),
        )
        .subcommand(SubCommand::with_name("version").about("Shows the version of BirdCTL"))
        .get_matches();
    match matches.subcommand() {
        ("connect", Some(subcommand)) => cli::connect(subcommand),
        ("version", Some(subcommand)) => cli::version(subcommand),
        _ => {}
    };
}
