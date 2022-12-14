mod cli;

use clap::Parser;
use cli::Args;
use color_eyre::Result;
use scraper::{Html, Selector};

const BASE_URL: &str = "https://glossary.infil.net/?t=";

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let Args { term } = Args::parse();
    let url = format!("{BASE_URL}{term}");
    let res = reqwest::Client::new()
        .get(&url)
        .send()
        .await?
        .error_for_status()?;
    let html = res.text().await?;

    if let Some(def) = get_def(&html) {
        println!("{def}");
        println!("[See {url}]")
    } else {
        println!("No results.")
    }

    Ok(())
}

fn get_def(html: &str) -> Option<String> {
    let doc = Html::parse_document(html);
    let selector = Selector::parse(r#"div[class="def"]"#).unwrap();

    doc.select(&selector).next().map(|def| {
        def.text()
            .collect::<String>()
            .split_whitespace()
            .map(|s| s.to_string() + " ")
            .collect()
    })
}
