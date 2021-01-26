use std::error::Error;

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("Compleet")
        .version("0.1.0")
        .author("Andrew Stanton-Nurse <contact@analogrelay.dev>")
        .arg(Arg::with_name("working-dir")
            .short("d")
            .long("working-dir")
            .value_name("DIR")
            .help("The git working dir, defaults to the current directory.")
            .takes_value(true))
        .arg(Arg::with_name("git-dir")
            .long("git-dir")
            .value_name("DIR")
            .help("The Git database directory, defaults to the '.git' subdirectory of the working-dir.")
            .takes_value(true))
        .subcommand(SubCommand::with_name("complete")
            .about("Performs line completion")
            .arg(Arg::with_name("line")
                .multiple(true)))
        .get_matches();

    println!("working-dir = {:?}", matches.value_of("working-dir"));
    println!("git-dir = {:?}", matches.value_of("git-dir"));

    let res = match matches.subcommand() {
        ("complete", Some(matches)) => complete(matches),
        (x, _) => Err(format!("unknown command: {}", x).into())
    };

    if let Err(e) = res {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}

fn complete(matches: &clap::ArgMatches) -> Result<(), Box<dyn Error>> {
    println!("line = {:?}", matches.values_of("line").map(|v| v.map(|v1| v1).collect::<Vec<&str>>()));
    todo!()
}
