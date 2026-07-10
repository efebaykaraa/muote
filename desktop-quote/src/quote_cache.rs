use std::path::PathBuf;

pub struct CachedQuote {
    pub text: String,
    pub author: String,
}

pub fn load_current_quote() -> CachedQuote {
    let cache_file = dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from("~/.cache"))
        .join("marxist_quote")
        .join("current_quote.txt");

    let raw_text = std::fs::read_to_string(cache_file).unwrap_or_default();
    parse_cached_quote(&raw_text)
}

fn parse_cached_quote(raw_text: &str) -> CachedQuote {
    if let Some((quote, author)) = raw_text.rsplit_once(" — ") {
        CachedQuote {
            text: quote.trim().trim_matches('"').to_string(),
            author: author.trim().to_string(),
        }
    } else {
        CachedQuote {
            text: raw_text.trim().to_string(),
            author: String::new(),
        }
    }
}
