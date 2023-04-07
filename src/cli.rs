use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    pub term: String,
    /// The number of characters to line-wrap after
    #[arg(short, long, default_value_t = 80)]
    pub wrap: usize,
}
