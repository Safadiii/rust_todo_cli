use chrono::Local;
use crossterm::event::{KeyCode, KeyEvent};
use humantime::parse_duration;
use color_eyre::Result;

use crate::{app::{AddTaskField, App, CmdMode, Focus, MainFocus}, char_to_byte_index, due_parse, storage::{TASK_PATH, save}, task::Recurrence};


fn parse_recurrence(input: &str) -> Option<Recurrence> {
    match input.trim().to_lowercase().as_str() {
        "" => None,
        "daily"   => Some(Recurrence::Daily),
        "weekly"  => Some(Recurrence::Weekly),
        "monthly" => Some(Recurrence::Monthly),
        other => parse_duration(other).ok().map(Recurrence::Custom),
    }
}

impl App {
    pub fn handle_popup(&mut self, key_event: KeyEvent) -> Result<()> {
        if !self.inputtingmode {
                match key_event.code {
                    KeyCode::Tab | KeyCode::Char('j') => {
                        match self.addtaskfield {
                            AddTaskField::Title => {self.addtaskfield = AddTaskField::Tags;},
                            AddTaskField::Due => {self.addtaskfield = AddTaskField::Recurring;},
                            AddTaskField::Tags => {self.addtaskfield = AddTaskField::Due;},
                            AddTaskField::Recurring => {self.addtaskfield = AddTaskField::Title;},
                        }
                        self.move_cursor_to_end();
                        self.clamp_cursor();
                    }
                    KeyCode::Char('k') => {
                        match self.addtaskfield {
                            AddTaskField::Title => {self.addtaskfield = AddTaskField::Recurring; self.move_cursor_to_end();},
                            AddTaskField::Due => {self.addtaskfield = AddTaskField::Tags; self.move_cursor_to_end();},
                            AddTaskField::Tags => {self.addtaskfield = AddTaskField::Title; self.move_cursor_to_end();},
                            AddTaskField::Recurring => {self.addtaskfield = AddTaskField::Due; self.move_cursor_to_end();},
                        }
                        self.clamp_cursor();
                    }
                    KeyCode::Char('c') => {
                        match self.addtaskfield {
                            AddTaskField::Title => {self.title_input.clear(); self.clamp_cursor();}
                            AddTaskField::Tags => {self.tags_input.clear(); self.clamp_cursor();}
                            AddTaskField::Due => {self.due_input.clear(); self.clamp_cursor();}
                            AddTaskField::Recurring => {self.recurrence_input.clear(); self.clamp_cursor();}
                        }
                    }
                    KeyCode::Char('q') | KeyCode::Esc => {self.focus = Focus::None},
                    KeyCode::Char('e') | KeyCode::Char('i') => {self.inputtingmode = true; self.move_cursor_to_end();}
                    KeyCode::Right => {self.char_index += 1; self.clamp_cursor();}
                    KeyCode::Left => {self.char_index = self.char_index.saturating_sub(1); self.clamp_cursor();}
                    KeyCode::Enter => {
                        if self.title_input.is_empty() {
                            return Ok(());
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
                    _ => {
                        self.inputtingmode = true;
                    }
            }
        } else {
            let input: &mut String = match self.addtaskfield {
                AddTaskField::Title => {&mut self.title_input},
                AddTaskField::Tags => {&mut self.tags_input},
                AddTaskField::Due => {&mut self.due_input},
                AddTaskField::Recurring => {&mut self.recurrence_input},
            };

            match key_event.code {
                KeyCode::Esc => {self.inputtingmode = false;},
                KeyCode::Char(c) => {
                    let byte_index = char_to_byte_index(input, self.char_index);
                    input.insert(byte_index, c);

                    self.char_index += 1;
                }
                KeyCode::Backspace => {
                    let mut c = self.char_index;

                    if c > 0 {
                        c = c.saturating_sub(1);
                        let bytes = char_to_byte_index(input, c);
                        input.remove(bytes);
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
        }
        Ok(())
    }
    pub fn handle_detailspopup(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.focus = Focus::None;
                self.mainfocus = MainFocus::Task;
                self.cmd = String::from("");
                self.cmd_index = 0;
                self.commandmode = CmdMode::None;
            }
            _ => {}
        }
        Ok(())
    }
}