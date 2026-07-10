mod app;

use simplelog::*;
use std::env;
use std::fs::File;

fn main() {
    let log_path = marxist_quote_core::config_dir().join("gui.log");
    let _ = std::fs::create_dir_all(marxist_quote_core::config_dir());

    let _ = CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            File::create(log_path).unwrap(),
        ),
    ]);

    log::info!("Starting Marxist Quote GUI...");

    let args: Vec<String> = env::args().collect();
    if args.iter().any(|a| a == "--fetch") {
        if let Err(e) = marxist_quote_core::fetch_quote() {
            log::error!("Error fetching quote: {}", e);
        }
        return;
    }

    // Start GUI
    let app = relm4::RelmApp::new("com.github.marxist_quote");
    app.run::<app::AppModel>(());
}
