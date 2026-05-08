use std::ascii::escape_default;

use chrono::Local;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use color_eyre::{Result, eyre::Ok};
use humantime::parse_duration;
use crate::{App, app::{AddTaskField, CmdMode, Focus, MainFocus}, char_to_byte_index, config::config::{Action, KeyBinding}, due_parse, events::popup::parse_recurrence, storage::{TASK_PATH, save}, task::Status};


enum clearType {
    Done,
    Overdue
}

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
        if self.inputtingmode {
            let _ = self.handle_inputting_mode(key_event);
            return Ok(());
        }

        let key = KeyBinding {
            code: key_event.code,
            modifier: key_event.modifiers,
        };

        if let Some(action) = self.config.keys.resolve(key, &self.mainfocus, &self.focus) {
            let _ = self.handle_action(action);
        }

        Ok(())
    }

    pub fn handle_action(&mut self, action: Action) -> Result<()> {
        match action {
            Action::Quit => self.exit = true,

            Action::Up => self.move_up(),
            Action::Down => if matches!(self.focus, Focus::None) {self.move_down()},
            Action::Search => {
                self.mainfocus = MainFocus::None;
                self.commandmode = CmdMode::Search;
                self.inputtingmode = true;
            },
            Action::Help => {
                if !matches!(self.focus, Focus::None) {
                    return Ok(());
                }
                self.focus = Focus::HelpPopup;
            }
            Action::Escape => {
                self.escape();
            }
            Action::SwitchFocus | Action::CategoryEnter => {
                self.switch_focus();
            }
            Action::TaskAdd => {
                if matches!(self.focus, Focus::None) {
                    self.focus = Focus::AddTaskPopup;
                    self.addtaskfield = AddTaskField::Title;                
                }
            }
            Action::CategoryAdd => {
                self.commandmode = CmdMode::AddingCategory;
                self.mainfocus = MainFocus::None;
                self.clamp_cursor();
            }
            Action::CategoryDelete => {
                todo!();
            }
            Action::CategoryClearOverdue => {
                self.clear(clearType::Overdue);
            }
            Action::CategoryClearDone => {
                self.clear(clearType::Done);
            }
            Action::CategorySort => {
                let category = self.categoryliststate.selected().and_then(|i| self.categories.get_mut(i)).unwrap();
                category.taskslist.sort_by_deadline();
            }
            Action::TaskComplete => {
                if let Some(task) = self.current_task_mut() {
                task.mark_completed();
                }
            }
            Action::TaskInProgress => {
                if let Some(task) = self.current_task_mut() {
                    task.status = Status::InProgress;
                }
            }
            Action::TaskDetails => {
                match self.focus {
                    Focus::None => self.focus = Focus::DetailsPopup,
                    _ => {}
                }
                
            }
            
            Action::PopupEditMode => {
                self.inputtingmode = true; 
                self.move_cursor_to_end();
            }
            Action::PopupCancel => {
                if self.inputtingmode {
                    self.inputtingmode = !self.inputtingmode;
                    self.move_cursor_to_end();
                } else {
                    self.escape();
                }
            }
            Action::PopupSubmit => {
                self.submit_popup();
            }
            Action::PopupNextField => {
                self.move_down();
            }
            Action::PopupPrevField => {
                self.move_up();
            }
            Action::PopupClear => {
                self.clear_field();
            }
            _ => {}
        }
        Ok(())
    }

    pub fn handle_inputting_mode(&mut self, key_event: KeyEvent) -> Result<()> {
        let input: &mut String = match self.addtaskfield {
            AddTaskField::Title => {&mut self.title_input},
            AddTaskField::Tags => {&mut self.tags_input},
            AddTaskField::Due => {&mut self.due_input},
            AddTaskField::Recurring => {&mut self.recurrence_input},
        };

        match key_event.code {
            KeyCode::Esc => {
                if matches!(self.mainfocus, MainFocus::None) {
                    self.cmd = String::new();
                    self.cmd_index = 0;
                    self.inputtingmode = false;
                    self.mainfocus = MainFocus::Categories;
                    self.focus = Focus::None;
                    return Ok(());
                }
                self.inputtingmode = false;
            },
            KeyCode::Char(c) => {
                let byte_index = char_to_byte_index(input, self.char_index);
                if matches!(self.mainfocus, MainFocus::None) {
                    let byte_index = char_to_byte_index(&self.cmd, self.cmd_index);
                    self.cmd.insert(byte_index, c);
                    self.cmd_index += 1;
                } else {
                    input.insert(byte_index, c);
                    self.char_index += 1;
                }

            }
            KeyCode::Backspace => {
                let mut c = match self.mainfocus {
                    MainFocus::None => self.cmd_index,
                    _ => self.char_index
                };

                if c > 0 {
                    c = c.saturating_sub(1);
                    let bytes = match self.mainfocus {
                        MainFocus::None => char_to_byte_index(&self.cmd, c),
                        _ => char_to_byte_index(input, c)
                    };
                    if matches!(self.mainfocus, MainFocus::None) {
                        self.cmd.remove(bytes);
                    } else {
                        input.remove(bytes);
                    }
                }
                self.clamp_cursor();
                self.move_cursor_to_end();
            }
            KeyCode::Enter => {
                match self.addtaskfield  {
                    AddTaskField::Title => {self.addtaskfield = AddTaskField::Tags;}
                    AddTaskField::Tags => {self.addtaskfield = AddTaskField::Due;}
                    AddTaskField::Due => {self.addtaskfield = AddTaskField::Recurring;}
                    AddTaskField::Recurring => {self.addtaskfield = AddTaskField::Title;}
                } 
                self.clamp_cursor(); 
                self.move_cursor_to_end();
            }
            _ => {}
        }
        Ok(())
    }


    /*
    Logic for handling moving between fields, tasks, or categories.
     */
    pub fn move_down(&mut self) {
        if matches!(self.focus, Focus::DetailsPopup | Focus::HelpPopup | Focus::Search) {
            return;
        }
        if matches!(self.focus, Focus::AddTaskPopup) {
            match self.addtaskfield {
                AddTaskField::Title => {self.addtaskfield = AddTaskField::Tags;},
                AddTaskField::Due => {self.addtaskfield = AddTaskField::Recurring;},
                AddTaskField::Tags => {self.addtaskfield = AddTaskField::Due;},
                AddTaskField::Recurring => {self.addtaskfield = AddTaskField::Title;},
            }
            self.move_cursor_to_end();
            self.clamp_cursor();
        } else {
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
    }

    pub fn move_up(&mut self) {
        if matches!(self.focus, Focus::DetailsPopup | Focus::HelpPopup | Focus::Search) {
            return;
        }
        if matches!(self.focus, Focus::AddTaskPopup) {
            match self.addtaskfield {
                AddTaskField::Title => {self.addtaskfield = AddTaskField::Recurring;},
                AddTaskField::Due => {self.addtaskfield = AddTaskField::Tags;},
                AddTaskField::Tags => {self.addtaskfield = AddTaskField::Title;},
                AddTaskField::Recurring => {self.addtaskfield = AddTaskField::Due;},
            }
            self.move_cursor_to_end();
            self.clamp_cursor();
        } else {
            match self.mainfocus {
                MainFocus::Categories => self.categoryliststate.select_previous(),
                MainFocus::Task => self.list_state.select_previous(),
                _ => {}
            }
        }
        
    }


    fn clear(&mut self, clear: clearType) {
        let category = self.categoryliststate.selected()
                        .and_then(|i| self.categories.get_mut(i))
                        .unwrap();

        match clear {
            clearType::Done => category.taskslist.clear_done(),
            clearType::Overdue => category.taskslist.clear_overdue(),
        }
    }

    fn escape(&mut self) {
        if !matches!(self.focus, Focus::None) {
            self.focus = Focus::None;
            self.mainfocus = MainFocus::Task;
            self.list_state.select_first();
            return;
        } else {
            self.mainfocus = MainFocus::Categories; 
            self.list_state.select(None);
        }
    }

    fn submit_popup(&mut self) {
        if self.title_input.is_empty() {
            return ;
        }

        let now = Local::now();
        let taskdue = if !self.due_input.is_empty() && due_parse(self.due_input.clone()) {
            let duration = parse_duration(&self.due_input).unwrap();
            Some(now + chrono::Duration::from_std(duration).unwrap())
        } else {
            None
        };
        let tags: Vec<String> = if self.tags_input.trim().is_empty() {
            vec![]
        } else {
            self.tags_input
                .split_whitespace()
                .map(|x| x.to_string())
                .collect()
        };
        if let Some(category) = self
            .categoryliststate
            .selected()
            .and_then(|i| self.categories.get_mut(i))
        {
            let recurrence = parse_recurrence(&self.recurrence_input);
            if let Some(edit_id) = self.editing_task_id {
                category.taskslist.update_task(edit_id, self.title_input.clone(), tags, taskdue);
            } else {
                category.taskslist.add(
                    self.title_input.as_str(),
                    tags,
                    taskdue,
                    recurrence,
                );
            }
            save(TASK_PATH, &self.categories);
        }
        self.title_input = String::new();
        self.tags_input = String::new();
        self.due_input = String::new();
        self.focus = Focus::None;
        self.addtaskfield = AddTaskField::Title;
        self.editing_task_id = None;
    }

    fn switch_focus(&mut self) {
        if !matches!(self.focus, Focus::None) {
            return;
        }
        match self.mainfocus {
            MainFocus::Categories => {
                self.mainfocus = MainFocus::Task; 
                self.list_state.select_first();
            },
            MainFocus::Task => {
                self.mainfocus = MainFocus::Categories;
                self.list_state.select(None);
            },
            _ => {}
        }
    }
    
    fn clear_field(&mut self) {
        match self.addtaskfield {
            AddTaskField::Title => {self.title_input.clear(); self.clamp_cursor();}
            AddTaskField::Tags => {self.tags_input.clear(); self.clamp_cursor();}
            AddTaskField::Due => {self.due_input.clear(); self.clamp_cursor();}
            AddTaskField::Recurring => {self.recurrence_input.clear(); self.clamp_cursor();}
        }
    }

    

    fn handle_search_events(&mut self, key_event: KeyEvent) -> Result<()> {
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