use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};

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
    630
}
fn default_quote_max_height() -> i32 {
    850
}
fn default_max_quote_chars() -> usize {
    500
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
                font_size: 24.0,
                text_color: "#ffffff".into(),
                bg_color: "#00000080".into(),
                bg_enabled: false,
                stroke_color: "#000000".into(),
                stroke_enabled: true,
                stroke_width: 2.0,
                shadow_color: "#000000ff".into(),
                shadow_enabled: true,
                shadow_offset: 2.0,
                shadow_blur: 0.0,
                shadow_size: 1.0,
                language: default_language(),
                bg_rounded: true,
                bg_fill: false,
                quote_h_align: HorizontalAlign::Center,
                quote_v_align: VerticalAlign::Top,
                author_h_align: HorizontalAlign::Center,
                author_v_align: VerticalAlign::Top,
                quote_x: 100,
                quote_y: 100,
                author_x: 100,
                author_y: 200,
                quote_max_width: 630,
                quote_max_height: 850,
                max_quote_chars: 500,
                position_hash: String::new(),
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
