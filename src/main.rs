mod cli;

use clap::Parser;
use cli::Args;

const BASE_URL: &str = "https://glossary.infil.net/?t=";

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let Args { term } = Args::parse();
    let res = reqwest::Client::new()
        .get(format!("{BASE_URL}{term}"))
        .send()
        .await?
        .error_for_status()?;
    let html = res.text().await?;
    println!("{html}");

    Ok(())
}
