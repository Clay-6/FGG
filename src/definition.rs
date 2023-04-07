use regex::Regex;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Definition {
    term: String,
    def: String,
    #[serde(default, rename = "altterm")]
    alt_terms: Vec<String>,
}

impl Definition {
    pub fn term(&self) -> &str {
        &self.term
    }

    pub fn alt_terms(&self) -> &[String] {
        &self.alt_terms
    }

    pub fn text(&self) -> String {
        let opening_regex = Regex::new("!<'").unwrap();
        let closing_regex = Regex::new("'>").unwrap();
        let doubles_regex = Regex::new("[A-z]+','").unwrap();
        let double_space = Regex::new("  ").unwrap();
        let tags_regex = Regex::new("<[^>]*>").unwrap();

        let mut tmp = opening_regex.replace_all(&self.def, "").to_string();
        tmp = closing_regex.replace_all(&tmp, "").into();
        tmp = doubles_regex.replace_all(&tmp, "").into();
        tmp = tags_regex.replace_all(&tmp, "").into();

        let text = tmp.split_whitespace().collect::<Vec<_>>();
        let mut txt = text.clone();
        for (i, w) in txt
            .iter_mut()
            .enumerate()
            .take_while(|(i, _)| *i < (text.len() - 1))
        {
            if *w.to_lowercase() == text[i + 1].to_lowercase() {
                *w = "";
            }
        }
        let text = txt.iter().map(|s| s.to_string() + " ").collect::<String>();
        double_space.replace_all(&text, " ").to_string()
    }

    pub fn text_wrapping(&self, limit: usize) -> String {
        let mut text = self
            .text()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let mut line_chars = 0;

        for w in text.iter_mut() {
            line_chars += w.len();
            if line_chars > limit {
                w.insert(0, '\n');
                line_chars = 0;
            }
        }

        text.join(" ")
    }
}
