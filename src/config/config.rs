use core::task;
use std::{fs, path::Path, collections::HashMap};

use crossterm::event::KeyCode;
use ratatui::style::Color;
use serde::{Deserialize, Serialize};

use crate::app::{Focus, MainFocus};




#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub colors: Colors,
    pub keys: KeyConfig,
}

pub struct AppConfig {
    pub ui: UiConfig,
    pub keys: KeyBindings,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    Quit,
    Help,
    Up,
    Down,
    Search,
    Escape,

    TaskAdd,
    TaskEdit,
    TaskDelete,
    TaskComplete,
    TaskInProgress,
    TaskDescription,

    CategoryAdd,
    CategoryDelete,
    CategorySort,
    CategoryEnter,

    PopupNextField,
    PopupPrevField,
    PopupSubmit,
    PopupCancel,
}
#[derive(Debug, Deserialize, Clone)]

pub struct KeyConfig {
    pub global: HashMap<String, String>,
    pub task: HashMap<String, String>,
    pub category: HashMap<String, String>,
    pub popup: HashMap<String, String>,
}

pub struct KeyBindings {
    pub global: HashMap<KeyCode, Action>,
    pub task: HashMap<KeyCode, Action>,
    pub category: HashMap<KeyCode, Action>,
    pub popup: HashMap<KeyCode, Action>,
}





#[derive(Debug, Deserialize, Clone)]
pub struct Colors {
    pub background: ColorConfig,
    pub foreground: ColorConfig,
    pub active: ColorConfig,
    pub inactive: ColorConfig,
}


#[derive(Debug, Deserialize, Serialize, Clone)]
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
            keys: KeyConfig {
                global: HashMap::new(),
                task: HashMap::new(),
                category: HashMap::new(),
                popup: HashMap::new(),
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

    let s = s.trim().to_lowercase();

    if s.chars().count() == 1 {
        return s.chars().next().map(KeyCode::Char).unwrap();
    }

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
        "tab" => KeyCode::Tab,
        _ => KeyCode::Null,
    }
}

#[derive(Debug, Clone)]

pub struct UiConfig {
    pub background: Color,
    pub foreground: Color,
    pub active: Color,
    pub inactive: Color,
}

impl From<Config> for UiConfig {
    fn from(cfg: Config) -> Self {
        Self {
            background: cfg.colors.background.parse_color(),
            foreground: cfg.colors.foreground.parse_color(),
            active: cfg.colors.active.parse_color(),
            inactive: cfg.colors.inactive.parse_color(),
        }
    }
}

//
// ==========================
// LOADER
// ==========================
//

pub fn load_config() -> AppConfig {
    let path = "config.toml";

    let cfg: Config = if Path::new(path).exists() {
        let text = fs::read_to_string(path).unwrap_or_default();
        toml::from_str(&text).unwrap_or_else(|_| Config::default())
    } else {
        Config::default()
    };

    AppConfig {
        ui: UiConfig::from(cfg.clone()),
        keys: KeyBindings::from(cfg),
    }
}

//FROM TRAIT

impl From<Config> for KeyBindings {
    fn from(cfg: Config) -> Self {
        fn build_map(map: &HashMap<String, String>, action_maps: &[(String, Action)])
        -> HashMap<KeyCode, Action>
        {
        
            let mut out = HashMap::new();

            for (action_name, action) in action_maps {
                if let Some(key_str) = map.get(action_name) {
                    let key = parse_key(key_str);
                    out.insert(key, *action);
                }
            }

            out
        }

        let global_actions = vec![
            ("quit".into(), Action::Quit),
            ("help".into(), Action::Help),
            ("up".into(), Action::Up),
            ("down".into(), Action::Down),
            ("search".into(), Action::Search),
            ("escape".into(), Action::Escape),
        ];    

        let task_actions = vec![
            ("add".into(), Action::TaskAdd),
            ("edit".into(), Action::TaskEdit),
            ("delete".into(), Action::TaskDelete),
            ("complete".into(), Action::TaskComplete),
            ("in_progress".into(), Action::TaskInProgress),
            ("description".into(), Action::TaskDescription),
        ];

        let category_actions = vec![
            ("add".into(), Action::CategoryAdd),
            ("delete".into(), Action::CategoryDelete),
            ("sort".into(), Action::CategorySort),
            ("enter_tasks".into(), Action::CategoryEnter),
        ];

        let popup_actions = vec![
            ("next".into(), Action::PopupNextField),
            ("prev".into(), Action::PopupPrevField),
            ("submit".into(), Action::PopupSubmit),
            ("cancel".into(), Action::PopupCancel),
        ];

        Self {
            global: build_map(&cfg.keys.global, &global_actions),
            task: build_map(&cfg.keys.task, &task_actions),     // same idea
            category: build_map(&cfg.keys.category, &category_actions),
            popup: build_map(&cfg.keys.popup, &popup_actions),
        }
    
    }
}

impl KeyBindings {
    pub fn resolve(
        &self,
        key: KeyCode,
        mainfocus: &MainFocus,
        focus: &Focus
    ) -> Option<Action> {

        if let Some(action) = self.global.get(&key) {
            return Some(*action);
        }
        match mainfocus {
            MainFocus::Task => {
                if let Some(action) = self.task.get(&key) {
                    return Some(*action);
                }
            },
            MainFocus::Categories => {
                if let Some(action) = self.category.get(&key) {
                    return Some(*action);
                }
            }
            _ => {}
        }

        if matches!(focus, Focus::AddTaskPopup | Focus::DetailsPopup) {
            if let Some(action) = self.popup.get(&key) {
                return Some(*action);
            }
        }
        None
    }
}