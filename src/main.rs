mod cli;

use clap::Parser;
use cli::Args;

const BASE_URL: &str = "https://glossary.infil.net/?t=";

fn main() {
    let args = Args::parse();
    println!("{args:?}")
}
