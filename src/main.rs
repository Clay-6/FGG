mod cli;
mod definition;

use clap::Parser;
use cli::Args;
use color_eyre::Result;
use definition::Definition;

const BASE_URL: &str = "https://glossary.infil.net";
const JSON_URL: &str = "https://glossary.infil.net/json/glossary.json";

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let Args { term, wrap } = Args::parse();
    let res = reqwest::Client::new().get(JSON_URL).send().await?;
    let json: Vec<Definition> = serde_json::from_str(&res.text().await?)?;

    if let Some(def) = json
        .iter()
        .find(|d| d.term().to_lowercase() == term.to_lowercase())
    {
        println!("{}", def.text_wrapping(wrap));
        println!("[{BASE_URL}/?t={term}]");
    } else if let Some(def) = json.iter().find(|d| {
        d.alt_terms()
            .iter()
            .any(|t| t.to_lowercase() == term.to_lowercase())
    }) {
        println!("{}", def.text_wrapping(wrap));
        println!("[{BASE_URL}/?t={term}]");
    } else {
        println!("No results.")
    }

    Ok(())
}
