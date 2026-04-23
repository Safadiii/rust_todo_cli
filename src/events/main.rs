use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use color_eyre::{Result, eyre::Ok};
use crate::{App, app::{CmdMode, Focus, MainFocus}};


impl App {
    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match self.focus {  
            Focus::AddTaskPopup => {self.handle_popup(key_event)?},
            Focus::None => {
                match self.mainfocus {
                    MainFocus::None => {self.handle_cmd_events(key_event)?}
                    MainFocus::Task => {self.handle_task_events(key_event)?}
                    MainFocus::Categories => {self.handle_category_events(key_event)?}
                    _ => {self.handle_main(key_event)?}
                }
            },
            Focus::Search => {self.handle_search_events(key_event)?}
            Focus::DetailsPopup => {self.handle_detailspopup(key_event)?}
            Focus::HelpPopup => {self.handle_helpkeys(key_event)?}
            _ => {}
        }
        Ok(())
    }

    pub fn handle_search_events(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Down | KeyCode::Char('j') => {
                self.searchliststate.select_next();   
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.searchliststate.select_previous();
            }
            KeyCode::Esc => {
                self.focus = Focus::None;
                self.mainfocus = MainFocus::Categories;
                self.cmd = String::from("");
                self.cmd_index = 0;
                self.commandmode = CmdMode::None;
            }
            _ => {}
        }
        Ok(())
    }

    pub fn handle_main(&mut self, key_event: KeyEvent) -> Result<()> {
        if key_event.kind != KeyEventKind::Press {
            return Ok(());
        }
        match key_event.code {
            KeyCode::Char('q') => self.exit = true,

            KeyCode::Char('S') => {
                self.mainfocus = MainFocus::None;
                self.commandmode = CmdMode::Search;
            }

  
            KeyCode::Char('H') => {
                self.focus = Focus::HelpPopup;
            }
            KeyCode::Tab => {
                self.mainfocus = MainFocus::Categories;
            }

            KeyCode::Esc => {
                match self.mainfocus {
                    MainFocus::None => {
                        self.mainfocus = MainFocus::Categories;
                        self.cmd = String::from("");
                    }
                    _ => {
                        self.mainfocus = MainFocus::Categories;
                        self.list_state.select(None);
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }
}