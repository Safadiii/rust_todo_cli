use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use color_eyre::{Result, config};

use crate::{app::{App, CmdMode, Focus, MainFocus}, category::Category, char_to_byte_index, search::{SearchConfig, search_fuzzy}};

impl App {
    pub fn handle_cmd_events(&mut self, key_event: KeyEvent) -> Result<()> {
        if key_event.kind != KeyEventKind::Press {
            return Ok(())
        }
        match key_event.code {
            KeyCode::Esc => {
                self.cmd = String::new();
                self.cmd_index = 0;
                self.mainfocus = MainFocus::Categories;
                self.commandmode = CmdMode::None;
            }
            KeyCode::Char(c) => {
                self.cmd.push(c);
                self.cmd_index += 1;
            }
            KeyCode::Backspace => {
                if self.cmd_index > 0 {
                    self.cmd_index = self.cmd_index.saturating_sub(1);
                    let bytes = char_to_byte_index(self.cmd.as_str(), self.cmd_index);
                    self.cmd.remove(bytes);
                }
            }
            KeyCode::Enter => {
                //Later change into a function to generalize
                match self.commandmode {
                    CmdMode::AddingCategory => {
                        self.categories.push(Category::new(std::mem::take(&mut self.cmd), None));
                        self.mainfocus = MainFocus::Categories;
                        self.commandmode = CmdMode::None;
                    },
                    CmdMode::AddingDescription => {
                        let input = self.cmd.clone();
                        self.current_task_mut().unwrap().add_description(input);
                        self.cmd.clear();
                        self.commandmode = CmdMode::None;
                        self.mainfocus = MainFocus::Categories;
                    },
                    CmdMode::Search => {
                        self.focus = Focus::Search;
                        self.mainfocus = MainFocus::SearchResults;
                        self.searchliststate.select_first();
                    }
                    _ => {}
                }
            }
            KeyCode::Right => {self.cmd_index += 1; self.clamp_cursor();}
            KeyCode::Left => {self.cmd_index = self.cmd_index.saturating_sub(1); self.clamp_cursor();}
            _ => {}
        }
        Ok(())
    }
}