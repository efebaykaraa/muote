pub struct CachedQuote {
    pub text: String,
    pub author: String,
}

pub fn load_current_quote() -> CachedQuote {
    let raw_text = std::fs::read_to_string(marxist_quote_core::cache_file_path()).unwrap_or_default();
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
