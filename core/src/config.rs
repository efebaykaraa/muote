use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum HorizontalAlign {
    Left,
    Center,
    Right,
}

impl Default for HorizontalAlign {
    fn default() -> Self {
        HorizontalAlign::Center
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum VerticalAlign {
    Top,
    Center,
    Bottom,
}

impl Default for VerticalAlign {
    fn default() -> Self {
        VerticalAlign::Top
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Appearance {
    pub font: String,
    pub font_size: f32,
    pub text_color: String,
    pub bg_color: String,
    pub bg_enabled: bool,
    pub stroke_color: String,
    pub stroke_enabled: bool,
    pub stroke_width: f32,
    pub shadow_color: String,
    pub shadow_enabled: bool,
    pub shadow_offset: f32,
    #[serde(default = "default_shadow_blur")]
    pub shadow_blur: f32,
    #[serde(default = "default_shadow_size")]
    pub shadow_size: f32,
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(default = "default_bg_rounded")]
    pub bg_rounded: bool,
    #[serde(default = "default_bg_fill")]
    pub bg_fill: bool,
    #[serde(default)]
    pub quote_h_align: HorizontalAlign,
    #[serde(default)]
    pub quote_v_align: VerticalAlign,
    #[serde(default)]
    pub author_h_align: HorizontalAlign,
    #[serde(default)]
    pub author_v_align: VerticalAlign,
    pub quote_x: i32,
    pub quote_y: i32,
    pub author_x: i32,
    pub author_y: i32,
    #[serde(default = "default_quote_max_width")]
    pub quote_max_width: i32,
    #[serde(default = "default_quote_max_height")]
    pub quote_max_height: i32,
    #[serde(default = "default_max_quote_chars")]
    pub max_quote_chars: usize,
    #[serde(default)]
    pub position_hash: String,
}

fn default_quote_max_width() -> i32 {
    1499
}

fn default_quote_max_height() -> i32 {
    315
}

fn default_max_quote_chars() -> usize {
    581
}

fn default_bg_rounded() -> bool {
    true
}

fn default_bg_fill() -> bool {
    false
}

fn default_shadow_blur() -> f32 {
    0.0
}

fn default_shadow_size() -> f32 {
    1.0
}

fn default_language() -> String {
    "ORIGINAL".into()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayArgs {
    pub appearance: Appearance,
}

impl Default for DisplayArgs {
    fn default() -> Self {
        Self {
            appearance: Appearance {
                font: "Inter".into(),
                font_size: 32.0,
                text_color: "#ffffff".into(),
                bg_color: "#811309ff".into(),
                bg_enabled: true,
                stroke_color: "#000000".into(),
                stroke_enabled: false,
                stroke_width: 2.0,
                shadow_color: "#000000ff".into(),
                shadow_enabled: false,
                shadow_offset: 0.5,
                shadow_blur: 0.0,
                shadow_size: 1.0,
                language: default_language(),
                bg_rounded: true,
                bg_fill: false,
                quote_h_align: HorizontalAlign::Center,
                quote_v_align: VerticalAlign::Bottom,
                author_h_align: HorizontalAlign::Right,
                author_v_align: VerticalAlign::Top,
                quote_x: 210,
                quote_y: 614,
                author_x: 1342,
                author_y: 966,
                quote_max_width: default_quote_max_width(),
                quote_max_height: default_quote_max_height(),
                max_quote_chars: default_max_quote_chars(),
                position_hash: "c1d49957c3456882f562c324ad89bb1ab63fd360".into(),
            },
        }
    }
}

impl DisplayArgs {
    pub fn calculate_position_hash(&self) -> String {
        let mut hasher = Sha1::new();
        let data = format!(
            "{}:{}:{}:{}",
            self.appearance.font,
            self.appearance.font_size,
            self.appearance.quote_max_width,
            self.appearance.quote_max_height
        );
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

#[derive(Debug, Clone)]
pub struct ConfigManager {
    app_name: String,
}

impl ConfigManager {
    pub fn new(app_name: impl Into<String>) -> Self {
        Self {
            app_name: app_name.into(),
        }
    }

    pub fn config_dir(&self) -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("~/.config"))
            .join(&self.app_name)
    }

    pub fn settings_path(&self) -> PathBuf {
        self.config_dir().join("settings.json")
    }

    pub fn load_hashed_json<T: for<'de> Deserialize<'de> + Default>(
        &self,
        path: &PathBuf,
    ) -> (T, String) {
        if let Ok(contents) = std::fs::read_to_string(path) {
            let mut json_part = String::new();
            let mut file_hash = String::new();

            for line in contents.lines() {
                if line.starts_with("hash:") {
                    file_hash = line["hash:".len()..].to_string();
                } else {
                    json_part.push_str(line);
                    json_part.push('\n');
                }
            }

            if let Ok(data) = serde_json::from_str(&json_part) {
                return (data, file_hash);
            }
        }
        (T::default(), String::new())
    }

    pub fn save_hashed_json<T: Serialize>(
        &self,
        path: &PathBuf,
        data: &T,
    ) -> anyhow::Result<String> {
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let json_str = serde_json::to_string_pretty(data)?;

        let mut hasher = Sha1::new();
        hasher.update(json_str.as_bytes());
        let new_hash = format!("{:x}", hasher.finalize());

        let final_content = format!("{}\nhash:{}", json_str, new_hash);
        std::fs::write(path, final_content)?;

        Ok(new_hash)
    }

    pub fn load_settings(&self) -> (DisplayArgs, String) {
        self.load_hashed_json(&self.settings_path())
    }

    pub fn save_settings(&self, data: &DisplayArgs) -> anyhow::Result<String> {
        self.save_hashed_json(&self.settings_path(), data)
    }
}

pub fn parse_color_to_rgba(hex: &str) -> (f64, f64, f64, f64) {
    let hex = hex.trim_start_matches('#');
    if hex.len() == 6 || hex.len() == 8 {
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255) as f64 / 255.0;
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255) as f64 / 255.0;
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255) as f64 / 255.0;
        let a = if hex.len() == 8 {
            u8::from_str_radix(&hex[6..8], 16).unwrap_or(255) as f64 / 255.0
        } else {
            1.0
        };
        (r, g, b, a)
    } else {
        (1.0, 1.0, 1.0, 1.0)
    }
}

pub fn rgba_to_hex(r: f64, g: f64, b: f64, a: f64) -> String {
    format!(
        "#{:02x}{:02x}{:02x}{:02x}",
        (r * 255.0).round().clamp(0.0, 255.0) as u8,
        (g * 255.0).round().clamp(0.0, 255.0) as u8,
        (b * 255.0).round().clamp(0.0, 255.0) as u8,
        (a * 255.0).round().clamp(0.0, 255.0) as u8
    )
}
