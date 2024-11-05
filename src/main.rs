mod cli;
mod lookup;

use clap::{CommandFactory, Parser};
use lookup::lookup_letter;

mod license {
    include!(concat!(env!("OUT_DIR"), "/license.rs"));
}

fn main() {
    let args: cli::Cli = cli::Cli::parse();
    if args.license {
        println!("{}", license::LICENSE);
        return;
    }
    if args.words.is_empty() {
        let _ = cli::Cli::command().print_help();
        return;
    }
    let strings: Vec<String> = args.words.join(" ").chars().map(|c| {
        let lower: String = c.to_lowercase().to_string();
        match lookup_letter(&lower, args.language) {
            Some(x) => format!("{} - {}",c, x),
            None => String::from(c)
        }
    }).collect();
    for line in strings {
        println!("{}", line);
    }
}
