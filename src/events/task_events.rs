use chrono::Local;
use crossterm::event::{KeyCode, KeyEvent};
use color_eyre::{Result};


use crate::{app::{AddTaskField, App, CmdMode, Focus, MainFocus}, format_short_duration, task::Status};


impl App {
    pub fn handle_task_events(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Char('q') => self.exit = true,

            KeyCode::Char('S') => {
                self.mainfocus = MainFocus::None;
                self.commandmode = CmdMode::Search;
            }

            KeyCode::Char('d') => {
                self.commandmode = CmdMode::AddingDescription;
                self.mainfocus = MainFocus::None;
            }

            KeyCode::Down | KeyCode::Char('j') => {
                self.list_state.select_next();
            }

            KeyCode::Up | KeyCode::Char('k') => {
                self.list_state.select_previous();
            }
            KeyCode::Delete => {
                let task_id = self.current_task_mut().map(|t| t.id);

                if let Some(id) = task_id {
                    if let Some(i) = self.categoryliststate.selected() {
                        if let Some(category) = self.categories.get_mut(i) {
                            category.taskslist.delete_task(id);
                        }
                    }
                }
            }   
            KeyCode::Char('H') => {
                self.focus = Focus::HelpPopup;
            }

            KeyCode::Char('a') => {
                self.focus = Focus::AddTaskPopup;
                self.addtaskfield = AddTaskField::Title;
            }
            KeyCode::Char('e') => {
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

            KeyCode::Enter => {
                self.focus = Focus::DetailsPopup;
            }

            KeyCode::Esc => {
                self.mainfocus = MainFocus::Categories;
                self.list_state.select(None);
            }
            
            KeyCode::Char('x') => {
                if let Some(task) = self.current_task_mut() {
                    task.mark_completed();
                }
            }

            KeyCode::Char('p') => {
                if let Some(task) = self.current_task_mut() {
                    task.status = Status::InProgress;
                }
            }

            KeyCode::Tab => {
                self.mainfocus = MainFocus::Categories;
                self.list_state.select(None);
            }
            _ => {}
        }

        Ok(())
    }
}