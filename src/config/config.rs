use core::task;
use std::{fs, path::Path, collections::HashMap};

use crossterm::event::{KeyCode, KeyModifiers};
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
    SwitchFocus,

    TaskAdd,
    TaskEdit,
    TaskDelete,
    TaskComplete,
    TaskInProgress,
    TaskDescription,
    TaskDetails,

    CategoryAdd,
    CategoryDelete,
    CategorySort,
    CategoryEnter,
    CategoryClearOverdue,
    CategoryClearDone,

    PopupNextField,
    PopupPrevField,
    PopupSubmit,
    PopupCancel,
    PopupEditMode,
    PopupClear,
}
#[derive(Debug, Deserialize, Clone)]

pub struct KeyConfig {
    pub global: HashMap<String, String>,
    pub task: HashMap<String, String>,
    pub category: HashMap<String, String>,
    pub popup: HashMap<String, String>,
}

pub struct KeyBindings {
    pub global: HashMap<KeyBinding, Action>,
    pub task: HashMap<KeyBinding, Action>,
    pub category: HashMap<KeyBinding, Action>,
    pub popup: HashMap<KeyBinding, Action>,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]

pub struct KeyBinding {
    pub code: KeyCode,
    pub modifier: KeyModifiers,
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

fn parse_key(s: &str) -> KeyBinding {

    let s = s.trim();

    if let Some(rest) = s.strip_prefix("ctrl+") {
        if let Some(c) = rest.chars().next() {
            return KeyBinding { code: KeyCode::Char(c.to_ascii_lowercase()), modifier: KeyModifiers::CONTROL }
        }
    }


    if s.len() == 1 {
        let c = s.chars().next().unwrap();

        return KeyBinding { code: KeyCode::Char(c), modifier: if c.is_ascii_uppercase() {
            KeyModifiers::SHIFT } else {
                KeyModifiers::NONE
            }
        }
    }
    match s.to_lowercase().as_str() {
        "up" => KeyBinding { code: KeyCode::Up, modifier: KeyModifiers::NONE },
        "down" => KeyBinding { code: KeyCode::Down, modifier: KeyModifiers::NONE },
        "left" => KeyBinding { code: KeyCode::Left, modifier: KeyModifiers::NONE },
        "right" => KeyBinding { code: KeyCode::Right, modifier: KeyModifiers::NONE },
        "enter" => KeyBinding { code: KeyCode::Enter, modifier: KeyModifiers::NONE },
        "esc" => KeyBinding { code: KeyCode::Esc, modifier: KeyModifiers::NONE },
        "tab" => KeyBinding { code: KeyCode::Tab, modifier: KeyModifiers::NONE },
        _ => KeyBinding { code: KeyCode::Null, modifier: KeyModifiers::NONE },
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
        -> HashMap<KeyBinding, Action>
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
            ("switch_focus".into(), Action::SwitchFocus),
        ];    

        let task_actions = vec![
            ("add".into(), Action::TaskAdd),
            ("edit".into(), Action::TaskEdit),
            ("delete".into(), Action::TaskDelete),
            ("complete".into(), Action::TaskComplete),
            ("in_progress".into(), Action::TaskInProgress),
            ("description".into(), Action::TaskDescription),
            ("details".into(), Action::TaskDetails),
        ];

        let category_actions = vec![
            ("add".into(), Action::CategoryAdd),
            ("delete".into(), Action::CategoryDelete),
            ("sort".into(), Action::CategorySort),
            ("enter_tasks".into(), Action::CategoryEnter),
            ("clear_overdue".into(), Action::CategoryClearOverdue),
            ("clear_done".into(), Action::CategoryClearDone),
        ];

        let popup_actions = vec![
            ("next".into(), Action::PopupNextField),
            ("prev".into(), Action::PopupPrevField),
            ("submit".into(), Action::PopupSubmit),
            ("cancel".into(), Action::PopupCancel),
            ("edit_mode".into(), Action::PopupEditMode),
            ("clear".into(), Action::PopupClear),
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
        key: KeyBinding,
        mainfocus: &MainFocus,
        focus: &Focus
    ) -> Option<Action> {

        if matches!(focus, Focus::AddTaskPopup | Focus::DetailsPopup) {
            if let Some(action) = self.popup.get(&key) {
                return Some(*action);
            }
        }

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


        None
    }
}