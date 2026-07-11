mod app;

use simplelog::*;
use std::env;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    if std::env::var_os("MARXIST_QUOTE_POSITION_PICKER").is_some()
        || args.iter().any(|arg| arg == "--position-picker")
    {
        let _ = position_containers::run();
        return;
    }

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

    let app = relm4::RelmApp::new("com.github.marxist_quote");
    app.run::<app::AppModel>(());
}
