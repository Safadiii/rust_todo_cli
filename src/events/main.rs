use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use color_eyre::{Result, eyre::Ok};
use crate::{App, app::{CmdMode, Focus, MainFocus}, config::config::Action};


impl App {
    // pub fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
    //     match self.focus {  
    //         Focus::AddTaskPopup => {self.handle_popup(key_event)?},
    //         Focus::None => {
    //             match self.mainfocus {
    //                 MainFocus::None => {self.handle_cmd_events(key_event)?}
    //                 MainFocus::Task => {self.handle_task_events(key_event)?}
    //                 MainFocus::Categories => {self.handle_category_events(key_event)?}
    //                 _ => {self.handle_main(key_event)?}
    //             }
    //         },
    //         Focus::Search => {self.handle_search_events(key_event)?}
    //         Focus::DetailsPopup => {self.handle_detailspopup(key_event)?}
    //         Focus::HelpPopup => {self.handle_helpkeys(key_event)?}
    //         _ => {}
    //     }
    //     Ok(())
    // }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        if key_event.kind != KeyEventKind::Press {
            return Ok(());
        }

        if let Some(action) = self.config.keys.resolve(key_event.code, &self.mainfocus, &self.focus) {
            let _ = self.handle_action(action);
        }

        Ok(())
    }

    pub fn handle_action(&mut self, action: Action) -> Result<()> {
        match action {
            Action::Quit => self.exit = true,

            Action::Up => self.move_up(),
            Action::Down => self.move_down(),
            Action::Search => {
                self.mainfocus = MainFocus::None;
                self.commandmode = CmdMode::Search;
            },
            Action::Help => {
                self.focus = Focus::HelpPopup;
            }
            Action::Escape => {
                self.focus = Focus::None;
                self.mainfocus = MainFocus::Categories;
                self.list_state.select(None);
            }
            Action::CategoryEnter => {
                self.mainfocus = MainFocus::Task;
                self.list_state.select_first();
            }
            _ => {}
        }
        Ok(())
    }

    pub fn move_down(&mut self) {
        match self.mainfocus {
            MainFocus::Categories => {
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
            },
            MainFocus::Task => {
                let category_us = self.categoryliststate.selected().unwrap();
                let cat = self.categories.get(category_us).unwrap();
                let len = cat.taskslist.tasks.len();

                if len == 0 {
                    self.list_state.select(None);
                } else {
                    let i = match self.list_state.selected() {
                        Some(i) => (i + 1) % len,
                        None => 0,
                    };
                    self.list_state.select(Some(i));
                }
            }
            _ => {}
        }
    }

    pub fn move_up(&mut self) {
        match self.mainfocus {
            MainFocus::Categories => self.categoryliststate.select_previous(),
            MainFocus::Task => self.list_state.select_previous(),
            _ => {}
        }
    }

    

    pub fn handle_search_events(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Down | KeyCode::Char('j') => {
                match self.mainfocus {
                    MainFocus::SearchResults => {self.categoryliststate.select_next();}
                    MainFocus::Task => {self.list_state.select_next();}
                    _ => {}
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.categoryliststate.select_previous();
            }
            KeyCode::Esc => {
                self.focus = Focus::None;
                self.mainfocus = MainFocus::Categories;
                self.cmd = String::from("");
                self.cmd_index = 0;
                self.commandmode = CmdMode::None;
                self.search_mode = false;
            }
            KeyCode::Enter => {
                match self.mainfocus {
                    MainFocus::SearchResults => {
                        self.mainfocus = MainFocus::Task;
                    },
                    MainFocus::Task => self.focus = Focus::DetailsPopup,
                    _ => {}
                }
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