use clap::{App, AppSettings, Arg, SubCommand};

mod cli;
mod config;
mod socket;

fn main() {
    let matches = App::new("BirdCTL")
        .version(config::VERSION)
        .author("Jeroen S. <jeroenschutrup@hotmail.nl")
        .about("CLI to interact with Bird routing daemon")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::with_name("socket_path")
                .short("s")
                .long("socket")
                .help("The path to the bird socket")
                .takes_value(true)
                .global(true)
                .default_value("/var/run/bird.ctl"),
        )
        .subcommand(
            SubCommand::with_name("connect").about("Test the connection to the Bird socket"),
        )
        .subcommand(SubCommand::with_name("version").about("Shows the version of BirdCTL"))
        .subcommand(SubCommand::with_name("run").about("Send a command to the bird server"))
        .get_matches();
    match matches.subcommand() {
        ("connect", Some(subcommand)) => cli::connect(subcommand),
        ("run", Some(subcommand)) => cli::run_command(subcommand),
        ("version", Some(subcommand)) => cli::version(subcommand),
        _ => {}
    };
}
