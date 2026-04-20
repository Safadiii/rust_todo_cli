use serde::{Deserialize, Serialize};
use crate::task::TaskList;

#[derive(Deserialize, Serialize)]
pub struct Category {
    pub title: String,
    pub taskslist: TaskList,
    pub parent: Option<Box<Category>>,
}

impl Category {
    pub fn new(title: String, parent: Option<Box<Category>>) -> Self {
        Self {
            title,
            taskslist: TaskList::new(),
            parent
        }
    }
}