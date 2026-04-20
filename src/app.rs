use std::ops::Add;

use ratatui::layout::Rect;
use ratatui::{DefaultTerminal, Frame};
use ratatui::widgets::ListState;
use crate::storage::save;
use crate::task::{TaskList, Task};
use crate::category::Category;
use color_eyre::Result;
use crate::{TASK_PATH,};

pub enum CmdMode {
    AddingCategory,
    None
}
pub enum Focus {
    None,
    AddTaskPopup,
    DetailsPopup,
    HelpPopup,
}

pub enum AddTaskField {
    Title,
    Tags,
    Due,
    Recurring
}

pub enum MainFocus {
    Task,
    Categories,
    None
}

pub struct App {
    pub exit: bool,
    pub taskslist: TaskList,
    pub list_state: ListState,
    pub focus: Focus,
    pub addtaskfield: AddTaskField,  //Selected field in add task popup
    pub title_input: String,
    pub tags_input: String,
    pub due_input: String,
    pub recurrence_input: String,
    pub char_index: usize,
    pub inputtingmode: bool,
    pub mainfocus: MainFocus,
    pub categories: Vec<Category>,
    pub categoryliststate: ListState,
    pub cmd: String,
    pub cmd_index: usize,
    pub commandmode: CmdMode,
    pub editing_task_id: Option<u32>,
}
impl App {
    pub fn new(categories: Vec<Category>) -> Self {
        let list_state = ListState::default();
        let mut categoryliststate = ListState::default();
        let tasks_list: TaskList = TaskList::new();
        categoryliststate.select(Some(0));

        Self {
            exit: false,
            taskslist: tasks_list,
            list_state,
            focus: Focus::None,
            addtaskfield: AddTaskField::Title,
            title_input: String::from("Hello"),
            tags_input: String::new(),
            due_input: String::new(),
            recurrence_input: String::new(),
            char_index: 0,
            inputtingmode: false,
            mainfocus: MainFocus::Categories,
            categories,
            categoryliststate,
            cmd: String::new(),
            cmd_index: 0,
            commandmode: CmdMode::None,
            editing_task_id: None,
        }
    }
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            match crossterm::event::read()? {
                crossterm::event::Event::Key(key_event) => self.handle_key_event(key_event)?,
                _ => {}
            }
        }
        save(TASK_PATH, &self.categories);
        Ok(())
    }
    //Get Current Task From Selected Category + List State
    pub fn current_task_mut(&mut self) -> Option<&mut Task> {
        self.categoryliststate
            .selected()
            .and_then(|cat_i| self.categories.get_mut(cat_i))
            .and_then(|cat| {
                self.list_state
                    .selected()
                    .and_then(|task_i| cat.taskslist.tasks.get_mut(task_i))
            })
    }

    pub fn current_task(&mut self) -> Option<&Task> {
        self.categoryliststate
            .selected()
            .and_then(|cat_i| self.categories.get_mut(cat_i))
            .and_then(|cat| {
                self.list_state
                    .selected()
                    .and_then(|task_i| cat.taskslist.tasks.get(task_i))
            })
    }
    //Cursor logic
    pub fn clamp_cursor(&mut self) {
        match self.focus {
            Focus::AddTaskPopup => {
                let len = match self.addtaskfield {
                    AddTaskField::Title => self.title_input.chars().count(),
                    AddTaskField::Tags => self.tags_input.chars().count(),
                    AddTaskField::Due => self.due_input.chars().count(),
                    AddTaskField::Recurring => self.recurrence_input.chars().count(),
                };
                self.char_index = self.char_index.clamp(0, len);
            }
            Focus::None => {
                self.cmd_index = self.cmd_index.clamp(0, self.cmd.chars().count())
            }
            _ => {}
        }

    }
    pub fn render_cursor(&self, frame: &mut Frame, areas: (Rect, Rect, Rect, Rect)) {
        let (title, tags, due, recurring) = areas;
        let (area, index) = match self.addtaskfield {
            AddTaskField::Title => (title, self.char_index),
            AddTaskField::Tags => (tags, self.char_index),
            AddTaskField::Due => (due, self.char_index),
            AddTaskField::Recurring => (recurring, self.char_index),
        };

        let x = area.x + 1 + index as u16;
        let y = area.y + 1;

        frame.set_cursor_position((x, y));
    }
    pub fn move_cursor_to_end(&mut self) {
        match self.focus {
            Focus::AddTaskPopup => {
                self.char_index = match self.addtaskfield {
                    AddTaskField::Title => {self.title_input.chars().count()}
                    AddTaskField::Tags => {self.tags_input.chars().count()}
                    AddTaskField::Due => {self.due_input.chars().count()}
                    AddTaskField::Recurring => {self.recurrence_input.chars().count()}
                };
            }
            _ => {}
        }

    }
}

