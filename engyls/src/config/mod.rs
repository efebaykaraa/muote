pub mod manager;
pub mod types;
pub mod utils;

pub use manager::ConfigManager;
pub use types::{Appearance, Author, AuthorsConfig, DisplayArgs};
pub use utils::{parse_color_to_rgba, rgba_to_hex};
