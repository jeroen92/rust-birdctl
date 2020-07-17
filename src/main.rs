use clap::{App, Arg, SubCommand};

mod cli;
mod socket;

const PATH: &str = "/home/jschutrup/Projects/rust/bird/files/socket/bird.ctl";

fn main() {
    let matches = App::new("BirdCTL")
        .version("0.0.1")
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
