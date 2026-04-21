/*

Data structures to hold tasks

TASK
TASKLIST
STATUS
*/

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    Done, InProgress 
}

#[derive(Serialize, Deserialize)]
pub enum Recurrence {
    Daily,
    Weekly,
    Monthly,
    Custom(std::time::Duration),
}
impl Recurrence {
    pub fn next_due(&self, from: DateTime<Local>) -> DateTime<Local> {
        match self {
            Recurrence::Daily => from + chrono::Duration::days(1),
            Recurrence::Monthly => from + chrono::Duration::days(30),
            Recurrence::Weekly => from + chrono::Duration::days(7),
            Recurrence::Custom(d) => from + chrono::Duration::from_std(*d).unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub tags: Vec<String>,
    pub status: Status,
    pub due: Option<DateTime<Local>>,
    pub description: String,
    pub recurrence: Option<Recurrence>,
}

impl Task {
    pub fn new(id: u32, title: &str, tags: Vec<String>, due: Option<DateTime<Local>>, recurrence: Option<Recurrence>) -> Self {
        Self {
            id, title: title.to_string(), tags, status: Status::InProgress, due, description: String::new(), recurrence
        }
    }
    pub fn mark_completed(&mut self) {
        if let Some(recurrence) = &self.recurrence {
            let base = self.due.unwrap_or_else(Local::now);

            self.due = Some(recurrence.next_due(base));
            self.status = Status::InProgress

        } else {
            self.status = Status::Done;
        }

    }
    pub fn add_description(&mut self, desc: String) {
        self.description = desc;
    }
}

/*

TaskList Structure to hold many tasks and its functions

*/

#[derive(Serialize, Deserialize)]
pub struct TaskList {
    pub tasks: Vec<Task>,
    pub next_id: u32,
}

impl TaskList {
    pub fn new() -> Self {
        let tasks: Vec<Task> = vec![];
        Self {
            tasks: tasks,
            next_id: 0,
        }
    }
    pub fn add(&mut self, title: &str, tags: Vec<String>, due: Option<DateTime<Local>>, recurrence: Option<Recurrence>) {
        self.next_id();
        let id = self.next_id;
        self.tasks.push(Task::new(id, title, tags, due, recurrence));
    }
    pub fn update_task(&mut self, id: u32, title: String, tags: Vec<String>, due: Option<DateTime<Local>>) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.title = title;
            task.tags = tags;
            task.due = due;
        }
    }
    pub fn _get_task(&mut self, id: u32) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|task| task.id == id)
    }
    pub fn clear_done(&mut self) {
        self.tasks.retain(|task| !matches!(task.status, Status::Done));
    }
    fn _clear_all(&mut self) {
        self.tasks.clear();
        self.next_id();
    }
    pub fn clear_overdue(&mut self) {
        let now = Local::now();
        self.tasks.retain(|task| {
            if let Some(due) = task.due {
                due > now
            } else {
                true
            }
        });
        self.next_id();
    }
    pub fn sort_by_deadline(&mut self) {
        let now = Local::now();

        self.tasks.sort_by(|a, b| {
            let a_diff = a.due.map(|d| d - now);
            let b_diff = b.due.map(|d| d - now);

            match (a_diff, b_diff) {
                (Some(a), Some(b)) => a.cmp(&b),
                (Some(_a), None) => std::cmp::Ordering::Less,
                (None, Some(_b)) => std::cmp::Ordering::Greater,
                (None, None) => std::cmp::Ordering::Equal,
            }
        });
    }
    fn next_id(&mut self) {
        self.next_id = self.tasks
            .iter()
            .map(|t| t.id)
            .max()
            .unwrap_or(0) + 1;
    }
    pub fn delete_task(&mut self, id: u32) {
        self.tasks.retain(|t| t.id != id);
    }
}
