pub mod config;

use config::{ConfigManager, DisplayArgs};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use wikiquote_fetcher::{QuotePool, QuotePoolStore, WikiquoteConfig};

pub const APP_NAME: &str = "marxist_quote";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Author {
    pub name: String,
    pub weight: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorsConfig {
    pub authors: Vec<Author>,
    #[serde(default = "default_show_weight_note")]
    pub show_weight_note: bool,
}

fn default_show_weight_note() -> bool {
    true
}

impl Default for AuthorsConfig {
    fn default() -> Self {
        Self {
            authors: vec![
                Author {
                    name: "Karl Marx".into(),
                    weight: 3,
                },
                Author {
                    name: "Friedrich Engels".into(),
                    weight: 2,
                },
                Author {
                    name: "Vladimir Lenin".into(),
                    weight: 2,
                },
            ],
            show_weight_note: true,
        }
    }
}

pub fn config_manager() -> ConfigManager {
    ConfigManager::new(APP_NAME)
}

pub fn config_dir() -> PathBuf {
    config_manager().config_dir()
}

pub fn settings_path() -> PathBuf {
    config_manager().settings_path()
}

pub fn authors_path() -> PathBuf {
    config_dir().join("authors.json")
}

pub fn quote_pool_store() -> QuotePoolStore {
    QuotePoolStore::new(config_dir().join("pools"))
}

pub fn cache_dir() -> PathBuf {
    dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from("~/.cache"))
        .join(APP_NAME)
}

pub fn cache_file_path() -> PathBuf {
    cache_dir().join("current_quote.txt")
}

pub fn load_authors() -> (AuthorsConfig, String) {
    config_manager().load_hashed_json(&authors_path())
}

pub fn save_authors(data: &AuthorsConfig) -> anyhow::Result<String> {
    config_manager().save_hashed_json(&authors_path(), data)
}

pub fn load_settings() -> (DisplayArgs, String) {
    config_manager().load_settings()
}

pub fn save_settings(data: &DisplayArgs) -> anyhow::Result<String> {
    config_manager().save_settings(data)
}

pub fn current_quote_exists() -> bool {
    std::fs::read_to_string(cache_file_path())
        .map(|text| !text.trim().is_empty())
        .unwrap_or(false)
}

pub fn parse_cached_quote(raw_text: &str) -> Option<(String, String)> {
    let (quote, author) = raw_text.rsplit_once(" — ")?;
    Some((
        quote.trim().trim_matches('"').to_string(),
        author.trim().to_string(),
    ))
}

pub fn read_cached_quote() -> anyhow::Result<Option<String>> {
    match std::fs::read_to_string(cache_file_path()) {
        Ok(text) => Ok(Some(text)),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(err) => Err(err.into()),
    }
}

pub fn clear_cached_quote() -> anyhow::Result<()> {
    match std::fs::remove_file(cache_file_path()) {
        Ok(()) => Ok(()),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(err) => Err(err.into()),
    }
}

pub fn remove_cached_quote_from_pool() -> anyhow::Result<()> {
    let raw_text = match std::fs::read_to_string(cache_file_path()) {
        Ok(text) => text,
        Err(_) => return Ok(()),
    };

    if let Some((quote, author)) = parse_cached_quote(&raw_text) {
        let store = quote_pool_store();
        if let Some(mut pool) = store.load(&author) {
            pool.quotes.retain(|candidate| candidate != &quote);
            let _ = store.save(&pool);
        }
    }

    clear_cached_quote()
}

fn weighted_author(authors: &[Author]) -> anyhow::Result<&str> {
    let total_weight: u32 = authors.iter().map(|a| a.weight).sum();
    if total_weight == 0 {
        anyhow::bail!("Total weight of authors is zero");
    }

    let mut rng = rand::rng();
    let mut chosen_weight = rng.random_range(0..total_weight);
    for author in authors {
        if chosen_weight < author.weight {
            return Ok(&author.name);
        }
        chosen_weight -= author.weight;
    }

    authors
        .first()
        .map(|author| author.name.as_str())
        .ok_or_else(|| anyhow::anyhow!("No authors configured"))
}

pub fn fetch_quote() -> anyhow::Result<()> {
    remove_cached_quote_from_pool()?;

    let (authors_cfg, _) = load_authors();
    let (mut settings_cfg, _) = load_settings();
    let authors: Vec<_> = authors_cfg
        .authors
        .into_iter()
        .filter(|author| !author.name.trim().is_empty())
        .collect();

    let selected_author = weighted_author(&authors)?;
    let current_hash = settings_cfg.calculate_position_hash();
    let max_chars = settings_cfg.appearance.max_quote_chars;

    println!(
        "Picking quote for {} (max chars: {}, hash: {})",
        selected_author, max_chars, current_hash
    );

    let store = quote_pool_store();
    let mut pool = store.load(selected_author).unwrap_or_else(|| QuotePool {
        key: selected_author.to_string(),
        quotes: Vec::new(),
    });

    if pool.key != selected_author || pool.quotes.is_empty() {
        pool.key = selected_author.to_string();
    }

    if settings_cfg.appearance.position_hash != current_hash || pool.quotes.is_empty() {
        println!("Hash mismatch or empty pool. Refetching from WikiQuote...");
        pool.quotes = wikiquote_fetcher::fetch_wikiquote_with_config(
            selected_author,
            &WikiquoteConfig {
                user_agent: "MarxistQuote/0.1.6".into(),
                ..WikiquoteConfig::default()
            },
        )?;
        settings_cfg.appearance.position_hash = current_hash;
        let _ = save_settings(&settings_cfg);
    }

    let mut rng = rand::rng();
    let mut chosen_quote = String::new();
    while !pool.quotes.is_empty() {
        let idx = rng.random_range(0..pool.quotes.len());
        let quote = pool.quotes.remove(idx);

        if quote.chars().count() <= max_chars {
            chosen_quote = quote;
            break;
        }

        println!(
            "Quote too long ({} chars), removing from pool.",
            quote.chars().count()
        );
    }

    if chosen_quote.is_empty() {
        anyhow::bail!(
            "No fitting quotes found for {} in current pool. Try resizing or wait for next fetch.",
            selected_author
        );
    }

    let _ = store.save(&pool);

    let display_quote =
        match wikiquote_fetcher::translate_quote(&chosen_quote, &settings_cfg.appearance.language) {
            Ok(translated) => translated,
            Err(err) => {
                eprintln!(
                    "Translation failed for language {}: {}",
                    settings_cfg.appearance.language, err
                );
                chosen_quote
            }
        };

    let formatted = format!("\"{}\" — {}", display_quote, selected_author);
    let _ = std::fs::create_dir_all(cache_dir());
    std::fs::write(cache_file_path(), &formatted)?;

    println!("Selected: {}", formatted);
    Ok(())
}

pub fn any_quote_fits_all_authors(max_chars: usize) -> anyhow::Result<bool> {
    let (authors_cfg, _) = load_authors();
    for author in &authors_cfg.authors {
        let quotes = wikiquote_fetcher::fetch_wikiquote(&author.name)?;
        if quotes.iter().any(|q| q.chars().count() <= max_chars) {
            return Ok(true);
        }
    }
    Ok(false)
}
