mod draw;
mod desktop;
mod quote_cache;
mod window;

fn main() -> anyhow::Result<()> {
    let (args, _) = marxist_quote_core::load_settings();
    let quote = quote_cache::load_current_quote();

    window::run_display(args, &quote.text, &quote.author);

    Ok(())
}
