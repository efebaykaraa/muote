mod draw;
mod quote_cache;
mod window;

use engyls::config::ConfigManager;

fn main() -> anyhow::Result<()> {
    let (args, _) = ConfigManager::load_settings();
    let quote = quote_cache::load_current_quote();

    window::run_display(args, &quote.text, &quote.author);

    Ok(())
}
