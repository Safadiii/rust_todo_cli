use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use color_eyre::{Result};

use crate::app::{App, CmdMode, MainFocus};

impl App {
    pub fn handle_category_events(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Char('q') => self.exit = true,

            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                let category = self.categoryliststate.selected().and_then(|i| self.categories.get_mut(i)).unwrap();
                category.taskslist.clear_overdue();
            }

            KeyCode::Char('d') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                let category = self.categoryliststate.selected().and_then(|i| self.categories.get_mut(i)).unwrap();
                category.taskslist.clear_done();
            }

            KeyCode::Char('S') => {
                self.mainfocus = MainFocus::None;
                self.commandmode = CmdMode::Search;
            }

            KeyCode::Down | KeyCode::Char('j') => {
                let len = self.categories.len();

                if len == 0 {
                    self.categoryliststate.select(None);
                } else {
                    let i = match self.categoryliststate.selected() {
                        Some(i) => (i + 1) % len,
                        None => 0,
                    };
                    self.categoryliststate.select(Some(i));
                }
            }

            KeyCode::Up | KeyCode::Char('k') => {
                self.categoryliststate.select_previous();
            }
            KeyCode::Char('D') => {
                let index = self.categoryliststate.selected().unwrap();

                self.categories.remove(index);
                self.categoryliststate.select_first();
            }

            KeyCode::Char('a') => {
                self.commandmode = CmdMode::AddingCategory;
                self.mainfocus = MainFocus::None;
                self.clamp_cursor();
            }

            KeyCode::Char('s') => {
                let category = self.categoryliststate.selected().and_then(|i| self.categories.get_mut(i)).unwrap();
                category.taskslist.sort_by_deadline();
            }

            KeyCode::Tab | KeyCode::Enter => {
                self.mainfocus = MainFocus::Task;
                self.list_state.select_first();
            }
            
            KeyCode::Esc => {
                self.mainfocus = MainFocus::None;
            }

            _ => {}
        }
        Ok(())
    }
}