mod app;

use simplelog::*;
use std::env;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    if std::env::var_os("MUOTE_POSITION_PICKER").is_some()
        || args.iter().any(|arg| arg == "--position-picker")
    {
        let _ = position_containers::run();
        return;
    }

    let log_path = muote_core::config_dir().join("gui.log");
    let _ = std::fs::create_dir_all(muote_core::config_dir());

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

    log::info!("Starting Muote GUI...");

    let app = relm4::RelmApp::new("com.github.muote");
    app.run::<app::AppModel>(());
}
