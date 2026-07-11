mod desktop;
mod draw;
mod quote_cache;
mod window;

fn main() -> anyhow::Result<()> {
    if std::env::args()
        .skip(1)
        .any(|arg| arg == "--fetch" || arg == "fetch")
    {
        return marxist_quote_core::fetch_quote();
    }

    let (args, _) = marxist_quote_core::load_settings();
    let quote = quote_cache::load_current_quote();

    window::run_display(args, &quote.text, &quote.author);

    Ok(())
}
