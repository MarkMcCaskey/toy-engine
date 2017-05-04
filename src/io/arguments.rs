//! Command line arguments
//! Used to change behavior when launching

use clap::{Arg, App, ArgMatches};

// Parses command line arguments
pub fn read_arguments<'input>() -> ArgMatches<'input> {
    App::new("waah")
        .version("0.0")
        .author("Mark McCaskey")
        .about("A toy game engine")
        .arg(Arg::with_name("trace")
                 .short("t")
                 .long("trace")
                 .help("Runs with verbose log messages")
                 .takes_value(false))
        .arg(Arg::with_name("debug")
                 .short("d")
                 .long("debug")
                 .help("Runs the engine in debug mode")
                 .takes_value(false))
        .get_matches()
}
