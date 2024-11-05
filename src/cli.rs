use clap::{Parser, ValueEnum};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Language {
    /// German spelling alphabet as defined in DIN 5009:2022-06
    German,
    /// English spelling alphabet as used by the NATO
    English,
}

#[derive(Parser)]
#[clap(author, version, about = "A small tool helping with spelling using spelling alphabets", long_about = None)]
pub struct Cli {
    /// The words which are to be spelled out
    #[arg(required = true)]
    pub words: Vec<String>,

    /// Which spelling alphabet should be used
    #[arg(short = 'l', long = "lang", value_enum, default_value_t = Language::German)]
    pub language: Language,
}
