mod cli;
mod lookup;

use clap::Parser;
use lookup::lookup_letter;

fn main() {
    let args: cli::Cli = cli::Cli::parse();
    let strings: Vec<String> = args.words.join(" ").chars().map(|c| {
        let lower: String = c.to_lowercase().to_string();
        match lookup_letter(&lower) {
            Some(x) => format!("{} - {}",c, x),
            None => String::from(c)
        }
    }).collect();
    for line in strings {
        println!("{}", line);
    }
}
