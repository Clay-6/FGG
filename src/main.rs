mod cli;

use clap::Parser;
use cli::Args;
use color_eyre::Result;
use regex::Regex;
use serde::Deserialize;

const BASE_URL: &str = "https://glossary.infil.net/?t=";
const JSON_URL: &str = "https://glossary.infil.net/json/glossary.json";

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let Args { term } = Args::parse();
    let res = reqwest::Client::new().get(JSON_URL).send().await?;
    let json: Vec<Definition> = serde_json::from_str(&res.text().await?)?;

    if let Some(def) = json
        .iter()
        .find(|d| d.term.to_lowercase() == term.to_lowercase())
    {
        println!("{}", def.text());
        println!("[{BASE_URL}{term}]");
    } else {
        println!("No results.")
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
struct Definition {
    term: String,
    def: String,
}

impl Definition {
    fn text(&self) -> String {
        let opening_regex = Regex::new("!<'").unwrap();
        let closing_regex = Regex::new("'>").unwrap();
        let doubles_regex = Regex::new("[A-z]+','").unwrap();
        let tags_regex = Regex::new("<[^>]*>").unwrap();

        let mut tmp = opening_regex.replace_all(&self.def, "").to_string();
        tmp = closing_regex.replace_all(&tmp, "").to_string();
        tmp = doubles_regex.replace_all(&tmp, "").to_string();
        tags_regex.replace_all(&tmp, "").to_string()
    }
}
