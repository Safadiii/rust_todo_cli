use std::{fs, path::Path};

use crossterm::event::KeyCode;
use ratatui::style::Color;
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize)]
pub struct Config {
    pub colors: Colors,
    pub keys: Keys,
}

#[derive(Debug, Deserialize)]
pub struct Colors {
    pub background: ColorConfig,
    pub foreground: ColorConfig,
    pub active: ColorConfig,
    pub inactive: ColorConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Keys {
    pub down: String,
    pub up: String,
    pub quit: String,
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ColorConfig {
    Named(String),
    Rgb { r: u8, g: u8, b: u8 },
    Indexed(u8),
}

impl Config {
    pub fn default() -> Config {
        Config {
            colors: Colors { background: ColorConfig::Named(String::from("green")), 
            foreground: ColorConfig::Rgb { r: 122, g: 23, b: 0 }, 
            active: ColorConfig::Named(String::new()) , inactive: ColorConfig::Indexed(250)
        },
            keys: Keys {
                down: "j".into(),
                up: "k".into(),
                quit: "q".into(),
            },       
        }
    }
}
impl ColorConfig {
    fn parse_color(&self) -> Color {
        match self {
            ColorConfig::Named(name) => match name.to_lowercase().as_str() {
                "green" => Color::Green,
                "red" => Color::Red,
                "blue" => Color::Blue,
                _ => Color::White,
            }
            ColorConfig::Rgb { r, g, b } => Color::Rgb(*r, *g, *b),
            ColorConfig::Indexed(i) => Color::Indexed(*i),
        }
    }
}

fn parse_key(s: &str) -> KeyCode {
    match s.to_lowercase().as_str() {
        "j" => KeyCode::Char('j'),
        "k" => KeyCode::Char('k'),
        "q" => KeyCode::Char('q'),
        "up" => KeyCode::Up,
        "down" => KeyCode::Down,
        "left" => KeyCode::Left,
        "right" => KeyCode::Right,
        "enter" => KeyCode::Enter,
        "esc" => KeyCode::Esc,
        _ => KeyCode::Null,
    }
}

#[derive(Debug, Clone)]
pub struct UiConfig {
    pub background: Color,
    pub foreground: Color,
    pub active: Color,
    pub inactive: Color,

    pub key_down: KeyCode,
    pub key_up: KeyCode,
    pub key_quit: KeyCode,
}

impl From<Config> for UiConfig {
    fn from(cfg: Config) -> Self {
        Self {
            background: cfg.colors.background.parse_color(),
            foreground: cfg.colors.foreground.parse_color(),
            active: cfg.colors.active.parse_color(),
            inactive: cfg.colors.inactive.parse_color(),

            key_down: parse_key(&cfg.keys.down),
            key_up: parse_key(&cfg.keys.up),
            key_quit: parse_key(&cfg.keys.quit),
        }
    }
}

//
// ==========================
// LOADER
// ==========================
//

pub fn load_config() -> UiConfig {
    let path = "config.toml";

    let raw = if Path::new(path).exists() {
        let text = fs::read_to_string(path)
            .unwrap_or_else(|_| String::new());

        toml::from_str(&text).unwrap_or_else(|_| Config::default())
    } else {
        // file doesn't exist → use default
        let default = Config::default();

        default
    };

    UiConfig::from(raw)
}