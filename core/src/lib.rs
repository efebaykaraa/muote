pub mod config;

use config::{ConfigManager, DisplayArgs};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use wikiquote_fetcher::{QuotePool, QuotePoolStore, WikiquoteConfig};

pub const APP_NAME: &str = "marxist_quote";
const TIMER_UNIT_NAME: &str = "marxist-quote-fetch.timer";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuoteIntervalUnit {
    Minutes,
    Hours,
    Days,
}

impl QuoteIntervalUnit {
    pub fn id(self) -> &'static str {
        match self {
            Self::Minutes => "minutes",
            Self::Hours => "hours",
            Self::Days => "days",
        }
    }

    pub fn from_id(id: &str) -> Self {
        match id {
            "hours" => Self::Hours,
            "days" => Self::Days,
            _ => Self::Minutes,
        }
    }

    fn systemd_suffix(self) -> &'static str {
        match self {
            Self::Minutes => "min",
            Self::Hours => "h",
            Self::Days => "d",
        }
    }
}

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

pub fn fetch_timer_path() -> PathBuf {
    if let Ok(path) = std::env::var("MARXIST_QUOTE_TIMER_PATH") {
        return PathBuf::from(path);
    }

    let mut candidates = Vec::new();
    if let Some(config_dir) = dirs::config_dir() {
        candidates.push(config_dir.join("systemd/user").join(TIMER_UNIT_NAME));
    }
    candidates.push(PathBuf::from("/etc/systemd/user").join(TIMER_UNIT_NAME));
    candidates.push(PathBuf::from("/usr/local/lib/systemd/user").join(TIMER_UNIT_NAME));
    candidates.push(PathBuf::from("/usr/lib/systemd/user").join(TIMER_UNIT_NAME));

    candidates
        .iter()
        .find(|path| path.exists())
        .cloned()
        .unwrap_or_else(|| PathBuf::from("/usr/lib/systemd/user").join(TIMER_UNIT_NAME))
}

pub fn fetch_timer_override_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("~/.config"))
        .join(format!("systemd/user/{}.d/override.conf", TIMER_UNIT_NAME))
}

pub fn remove_fetch_timer_override() -> anyhow::Result<()> {
    let path = fetch_timer_override_path();
    match std::fs::remove_file(&path) {
        Ok(()) => Ok(()),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(err) => Err(err.into()),
    }
}

pub fn load_quote_timer_interval() -> anyhow::Result<(u32, QuoteIntervalUnit)> {
    let contents = std::fs::read_to_string(fetch_timer_path())?;
    let mut in_timer_section = false;
    let mut calendar_interval = None;

    for raw_line in contents.lines() {
        let line = raw_line.trim();
        if line.starts_with('[') && line.ends_with(']') {
            in_timer_section = line == "[Timer]";
            continue;
        }

        if !in_timer_section || line.starts_with('#') || line.starts_with(';') {
            continue;
        }

        if let Some(value) = line.strip_prefix("OnUnitActiveSec=") {
            if let Some(interval) = parse_systemd_interval(value.trim()) {
                return Ok(interval);
            }
        }

        if let Some(value) = line.strip_prefix("OnCalendar=") {
            calendar_interval = parse_calendar_interval(value.trim());
        }
    }

    Ok(calendar_interval.unwrap_or((1, QuoteIntervalUnit::Days)))
}

pub fn apply_quote_timer_interval(value: u32, unit: QuoteIntervalUnit) -> anyhow::Result<()> {
    let value = value.max(1);
    let timer_path = fetch_timer_path();
    let contents = std::fs::read_to_string(&timer_path)?;
    let mut output = Vec::new();
    let mut in_timer_section = false;
    let mut saw_timer_section = false;
    let mut inserted_interval = false;

    for raw_line in contents.lines() {
        let line = raw_line.trim();
        if line.starts_with('[') && line.ends_with(']') {
            if in_timer_section && !inserted_interval {
                output.push(format!(
                    "OnUnitActiveSec={}{}",
                    value,
                    unit.systemd_suffix()
                ));
                inserted_interval = true;
            }

            in_timer_section = line == "[Timer]";
            saw_timer_section |= in_timer_section;
            output.push(raw_line.to_string());
            continue;
        }

        if in_timer_section
            && (line.starts_with("OnCalendar=") || line.starts_with("OnUnitActiveSec="))
        {
            continue;
        }

        output.push(raw_line.to_string());
    }

    if saw_timer_section && !inserted_interval {
        output.push(format!(
            "OnUnitActiveSec={}{}",
            value,
            unit.systemd_suffix()
        ));
    } else if !saw_timer_section {
        if !output.last().map(|line| line.is_empty()).unwrap_or(false) {
            output.push(String::new());
        }
        output.push("[Timer]".to_string());
        output.push(format!(
            "OnUnitActiveSec={}{}",
            value,
            unit.systemd_suffix()
        ));
    }

    write_timer_file(&timer_path, &format!("{}\n", output.join("\n")))?;
    remove_fetch_timer_override()?;
    Ok(())
}

fn write_timer_file(path: &PathBuf, contents: &str) -> anyhow::Result<()> {
    match std::fs::write(path, contents) {
        Ok(()) => return Ok(()),
        Err(err) if err.kind() == std::io::ErrorKind::PermissionDenied => {}
        Err(err) => return Err(err.into()),
    }

    let temp_path = std::env::temp_dir().join(format!(
        "marxist-quote-fetch-{}-{}.timer",
        std::process::id(),
        chrono_like_timestamp()
    ));
    std::fs::write(&temp_path, contents)?;

    let status = std::process::Command::new("pkexec")
        .arg("install")
        .arg("-m")
        .arg("644")
        .arg(&temp_path)
        .arg(path)
        .status();
    let _ = std::fs::remove_file(&temp_path);

    match status {
        Ok(status) if status.success() => Ok(()),
        Ok(status) => anyhow::bail!("pkexec install exited with {}", status),
        Err(err) => Err(err.into()),
    }
}

fn chrono_like_timestamp() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or(0)
}

fn parse_calendar_interval(value: &str) -> Option<(u32, QuoteIntervalUnit)> {
    match value {
        "minutely" => Some((1, QuoteIntervalUnit::Minutes)),
        "hourly" => Some((1, QuoteIntervalUnit::Hours)),
        "daily" => Some((1, QuoteIntervalUnit::Days)),
        _ => None,
    }
}

fn parse_systemd_interval(value: &str) -> Option<(u32, QuoteIntervalUnit)> {
    let value = value.trim();
    let digits: String = value.chars().take_while(|c| c.is_ascii_digit()).collect();
    let amount = digits.parse::<u32>().ok()?.max(1);
    let unit_text = value[digits.len()..].trim();

    match unit_text {
        "min" | "minute" | "minutes" | "m" => Some((amount, QuoteIntervalUnit::Minutes)),
        "h" | "hr" | "hour" | "hours" => Some((amount, QuoteIntervalUnit::Hours)),
        "d" | "day" | "days" => Some((amount, QuoteIntervalUnit::Days)),
        "" | "s" | "sec" | "second" | "seconds" => {
            Some((amount.div_ceil(60), QuoteIntervalUnit::Minutes))
        }
        _ => None,
    }
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

    let display_quote = match wikiquote_fetcher::translate_quote(
        &chosen_quote,
        &settings_cfg.appearance.language,
    ) {
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
