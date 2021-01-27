use std::error::Error;

use clap::{App, Arg, SubCommand};
use context::CompletionContext;

mod context;

fn main() {
    if let Err(e) = real_main() {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}

fn real_main() -> Result<(), Box<dyn Error>> {
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
            .about("Performs completion")
            .arg(Arg::with_name("line")
                .multiple(true))
            .arg(Arg::with_name("position")
                .short("p")
                .long("position")
                .help("The position of the cursor in the line being completed.")
                .takes_value(true)))
        .get_matches();

    let working_dir = match matches.value_of("working-dir") {
        Some(s) => s.into(),
        None => std::env::current_dir()?.to_path_buf(),
    };

    let git_dir = match matches.value_of("git-dir") {
        Some(s) => s.into(),
        None => {
            let mut p = working_dir.clone();
            p.push(".git");
            p
        }
    };

    let context = CompletionContext::new(working_dir, git_dir);

    match matches.subcommand() {
        ("complete", Some(matches)) => complete(context, matches),
        (x, _) => Err(format!("unknown command: {}", x).into())
    }
}

fn complete(context: CompletionContext, matches: &clap::ArgMatches) -> Result<(), Box<dyn Error>> {
    let line: Vec<_> = matches.values_of("line").map(|v| v.collect()).unwrap_or(vec![]);
    let line = line.as_slice().join(" ");
    let position = matches.value_of("position").map(|v| v.parse()).unwrap_or(Ok(line.len()))?;
    let completions = context.complete_line(&line, position)?;
    for completion in completions {
        println!("{}", completion);
    }
    Ok(())
}
