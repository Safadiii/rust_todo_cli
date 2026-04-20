use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use color_eyre::{Result};
use crate::{App, app::{AddTaskField, CmdMode, Focus, MainFocus}, format_short_duration, task::Status};
use crate::Local;
impl App {
    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match self.focus {  
            Focus::AddTaskPopup => {self.handle_popup(key_event)?},
            Focus::None => {
                match self.mainfocus {
                    MainFocus::None => {self.handle_cmd_events(key_event)?}
                    _ => {self.handle_main(key_event)?}
                }
            },
            Focus::DetailsPopup => {self.handle_detailspopup(key_event)?}
            Focus::HelpPopup => {self.handle_helpkeys(key_event)?}
        }
        Ok(())
    }

    pub fn handle_main(&mut self, key_event: KeyEvent) -> Result<()> {
        if key_event.kind != KeyEventKind::Press {
            return Ok(());
        }
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

            KeyCode::Down | KeyCode::Char('j') => {
                match self.mainfocus {
                    MainFocus::Task => {
                        let tasks_len = self
                            .categoryliststate
                            .selected()
                            .and_then(|i| self.categories.get(i))
                            .map(|cat| cat.taskslist.tasks.len())
                            .unwrap_or(self.taskslist.tasks.len());

                        let i = match self.list_state.selected() {
                            Some(i) => i + 1,
                            None => 0,
                        };

                        if i < tasks_len {
                            self.list_state.select(Some(i));
                        }
                    },
                    MainFocus::Categories => {
                        let i = match self.categoryliststate.selected() {
                            Some(i) => i + 1,
                            None => 0,
                        };
                    
                    if i < self.categories.len() {
                        self.categoryliststate.select(Some(i));
                    }
                    }
                    _ => {}
            }
            }

            KeyCode::Up | KeyCode::Char('k') => {
                match self.mainfocus {
                    MainFocus::Task => {
                        let i = match self.list_state.selected() {
                            Some(i) => i.saturating_sub(1),
                            None => 0,
                        };

                        self.list_state.select(Some(i));
                    },
                    MainFocus::Categories => {
                        let i = match self.categoryliststate.selected() {
                            Some(i) => i.saturating_sub(1),
                            None => 0,
                        };
                        self.categoryliststate.select(Some(i));
                    }
                    _ => {}
                }
            }
            KeyCode::Char('D') => {
                match self.mainfocus {
                    MainFocus::Categories => {
                        let index = self.categoryliststate.selected().unwrap();

                        self.categories.remove(index);
                        self.categoryliststate.select(Some(0));
                    }
                    MainFocus::Task => {
                        let task_id = self.current_task_mut().map(|t| t.id);

                        if let Some(id) = task_id {
                            if let Some(i) = self.categoryliststate.selected() {
                                if let Some(category) = self.categories.get_mut(i) {
                                    category.taskslist.delete_task(id);
                                }
                            }
                        }}
                    _ => {}
                }
            }
            KeyCode::Char('H') => {
                self.focus = Focus::HelpPopup;
            }

            KeyCode::Char('a') => {
                match self.focus {
                    Focus::None => {
                        self.focus = Focus::AddTaskPopup;
                        self.addtaskfield = AddTaskField::Title;
                    }
                    Focus::AddTaskPopup => {
                        self.focus = Focus::None;
                    }
                    _ => {}
                }
            }

            KeyCode::Char('e') => {
                match self.mainfocus {
                    MainFocus::Task => {
                        if let Some(task) = self.current_task() {
                            let task_id = task.id;
                            let title = task.title.clone();
                            let tags = task.tags.clone();
                            let due = task.due;

                            self.editing_task_id = Some(task_id);
                            self.title_input = title;
                            self.tags_input = tags.join(" ");
                            self.due_input = due
                                .map(|d| {
                                    let now = Local::now();
                                    let duration = if d > now {
                                        (d - now).to_std().unwrap_or_default()
                                    } else {
                                        (now - d).to_std().unwrap_or_default()
                                    };
                                    format_short_duration(duration)
                                })
                                .unwrap_or_default();
                            self.focus = Focus::AddTaskPopup;
                        }
                    }
                    _ => {}
                }
            }

            KeyCode::Tab => {
                match self.mainfocus {
                    MainFocus::None => {
                        self.mainfocus = MainFocus::Categories;
                    }
                    MainFocus::Task => {
                        self.mainfocus = MainFocus::Categories;
                        self.list_state.select(None);
                    }
                    MainFocus::Categories => {
                        self.mainfocus = MainFocus::Task;
                        self.list_state.select(Some(0));
                    },
                }
            }

            KeyCode::Char('C') => {
                match self.mainfocus {
                    MainFocus::Categories | MainFocus::Task => {
                        self.commandmode = CmdMode::AddingCategory;
                        self.mainfocus = MainFocus::None;
                        self.clamp_cursor();
                    }
                    _ => {}
                }
            }
            KeyCode::Char('S') => {
                let category = self.categoryliststate.selected().and_then(|i| self.categories.get_mut(i)).unwrap();
                category.taskslist.sort_by_deadline();
            }

            KeyCode::Enter => {
                match self.mainfocus {
                    MainFocus::Task => {self.focus = Focus::DetailsPopup;}
                    MainFocus::Categories => {self.mainfocus = MainFocus::Task; self.list_state.select(Some(0));}
                    _ => {}
                }
            }

            KeyCode::Esc => {
                match self.focus {
                    Focus::DetailsPopup => {self.focus =Focus::None}
                    _ => {self.focus = Focus::None} 
                }
                match self.mainfocus {
                    MainFocus::Categories => self.mainfocus = MainFocus::None,
                    MainFocus::None => {
                        self.mainfocus = MainFocus::Categories;
                        self.cmd = String::new();
                    }
                    MainFocus::Task => {
                        self.mainfocus = MainFocus::Categories;
                    }
                }
            }
            KeyCode::Char('x') => {
                match self.mainfocus {
                    MainFocus::Task => {
                        if let Some(task) = self.current_task_mut() {
                            task.mark_completed();
                        }
                    }
                    _ => {}
                }
            }
            KeyCode::Char('p') => {
                match self.mainfocus {
                    MainFocus::Task => {
                        if let Some(task) = self.current_task_mut() {
                            task.status = Status::InProgress;
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        Ok(())
    }
}