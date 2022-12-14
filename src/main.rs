mod cli;

use clap::Parser;
use cli::Args;
use scraper::{Html, Selector};

const BASE_URL: &str = "https://glossary.infil.net/?t=";

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let Args { term } = Args::parse();
    let url = format!("{BASE_URL}{term}");
    let res = reqwest::Client::new()
        .get(&url)
        .send()
        .await?
        .error_for_status()?;
    let html = res.text().await?;

    Ok(())
}
