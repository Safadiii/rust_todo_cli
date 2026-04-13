use std::fs;
use std::io;
use std::fmt;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use color_eyre::Result;
use chrono::{DateTime, Local};
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyEventKind;
use humantime::format_duration;
use ratatui::DefaultTerminal;
use ratatui::style::Stylize;
use ratatui::widgets::Widget;
use serde::{Serialize, Deserialize};
use humantime::parse_duration;
use ratatui::{Frame, text::Line};

const TASK_PATH: &str = "tasks.json";

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
enum Status {
    Done, InProgress 
}
enum AddMode {
    None,
    Title,
    Tags,
    Due
}
enum ListMode {
    Extended,
    Standard,
    Sorted
}
enum ClearMode {
    All,
    Done,
    OverDue
}
impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Done => write!(f, "Done"),
            Status::InProgress => write!(f, "In Progress"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: u32,
    title: String,
    tags: Vec<String>,
    status: Status,
    due: Option<DateTime<Local>>,
}

impl Task {
    fn create(id: u32, title: &str, tags: Vec<String>, due: Option<DateTime<Local>>) -> Self {
        Self {
            id, title: title.to_string(), tags, status: Status::InProgress, due
        }
    }
    fn mark_completed(&mut self) {
        self.status = Status::Done;
    }
}
#[derive(Serialize, Deserialize)]
struct TaskList {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TaskList {
    fn add(&mut self, title: &str, tags: Vec<String>, due: Option<DateTime<Local>>) {
        self.next_id();
        let id = self.next_id;
        self.tasks.push(Task::create(id, title, tags, due));
    }
    fn list(&self) {
        if self.tasks.is_empty() {
            println!("No tasks to show add a task by running add")
        } else {
            for task in &self.tasks {
                let id = &task.id;
                let title = &task.title;
                let progress = &task.status;
                if let Some(due) = task.due {
                    let now = Local::now();

                    if due > now {
                        let diff = due - now;
                        let std_dur = diff.to_std().unwrap();
                        println!("{id}- {title} {progress} | Due: {}", format_duration(std_dur));
                    } else {
                        println!("{id}- {title} {progress} | OverDue: {}", due.format("%Y-%m-%d %H:%M"));
                    }
                } else {
                    println!("{id}- {title} {progress}");
                }
            }
        }
    }
    fn list_extended(&self) {
        if self.tasks.is_empty() {
            println!("No tasks to show add a task by running add")
        } else {
            for task in &self.tasks {
                let id = &task.id;
                let title = &task.title;
                let progress = &task.status;
                let tags = &task.tags.join(", ");
                println!("{id}- {title} | {progress} | {tags} ") // list tags here
            }
        }
    }
    fn get_task(&mut self, id: u32) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|task| task.id == id)
    }
    fn clear_done(&mut self) {
        self.tasks.retain(|task| task.status != Status::Done);
    }
    fn clear_all(&mut self) {
        self.tasks.clear();
        self.next_id();
    }
    fn clear_overdue(&mut self) {
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
    fn save_to_json(&self, path: &str) {
        let json = serde_json::to_string_pretty(self).expect("Could not serialize tasks list.");

        let mut file = OpenOptions::new() 
            .write(true)
            .read(true)
            .create(true)
            .truncate(true)
            .open(path)
            .expect("Failed to open file.");

        file.write_all(json.as_bytes())
            .expect("Could not write to file.");
    }
    fn load(path: &str) -> Self {
        if !Path::new(path).exists() {
            let tasks = TaskList {
                tasks: vec![],
                next_id: 1,
            };

            tasks.save_to_json(path);
            return tasks;
        } 
        let data = fs::read_to_string(path).unwrap_or_else(|_| {
            println!("Failed to create file, creating new one.");
            return String::new();
        });

        if data.trim().is_empty() {
            TaskList { tasks: vec![], next_id: 1, };
        }

        serde_json::from_str(&data).unwrap_or_else(|_| {
            println!("Corrupted file, couldn't read.");
            TaskList { tasks: vec![], next_id: 1 }
        })
    }
    fn sort_by_deadline(&mut self) {
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
    fn delete_task(&mut self, id: u32) {
        self.tasks.retain(|t| t.id != id);
    }
}


//HELPER FUNCTIONS


fn take_input(tasks: &mut TaskList) {

    //Input from the CLI
    let mut input: String = String::new();

    io::stdin().read_line(&mut input).expect("Could not read line");

    let mut parts = input.split_whitespace();

    let cmd = parts.next();
    let args: Vec<String> = parts.map(str::to_owned).collect();

    match cmd {
        Some("add") => {
            //initialize the empty vector strings
            let mut title_parts: Vec<String> = Vec::new();
            let mut tag_parts: Vec<String> = Vec::new();

            let mut mode = AddMode::None;

            let mut due: Option<DateTime<Local>> = None;

            for arg in &args {
                match arg.as_str() {
                    "-t" | "-title" => mode = AddMode::Title,
                    "-tg" | "-tags" => mode = AddMode::Tags,
                    "-due" => mode = AddMode::Due,
                    _ => match mode {
                        AddMode::Title => title_parts.push(arg.clone()),
                        AddMode::Tags => tag_parts.push(arg.clone()),
                        AddMode::Due => {
                            match parse_duration(arg) {
                                Ok(duration) => {
                                    let now = Local::now();
                                    due = Some(now + chrono::Duration::from_std(duration).unwrap());
                                }
                                Err(_) => {
                                    println!("Error parsing time.")
                                }
                            }
                        }
                        AddMode::None => {}
                    }
                }
            }

            let title = title_parts.join(" ");

            if !title.is_empty() {
                tasks.add(&title, tag_parts, due);
                tasks.save_to_json(TASK_PATH);
            } else {
                println!("Error add a title using -t flag");
            }
        }

        Some("help") => {
            display_commands();
        },
        Some("list") | Some("ls") => {
            let mut mode = ListMode::Standard;
            for arg in &args {
                match arg.as_str() {
                    "-ext" | "-e" | "-extended" => mode = ListMode::Extended,
                    "-sort" | "-sorted" => mode = ListMode::Sorted,
                    _ => mode = ListMode::Standard,
                }
            }
            match mode {
                ListMode::Extended => tasks.list_extended(),
                ListMode::Standard => tasks.list(),
                ListMode::Sorted => {tasks.sort_by_deadline(); tasks.list();}
            }
        },
        Some("exit") => {
            std::process::exit(23);
        },
        Some("done") => {
            if let [first] = args.as_slice() {
                match first.parse::<u32>() {
                    Ok(num) => {
                        match tasks.get_task(num) {
                            Some(task) => {
                                task.mark_completed();
                                tasks.save_to_json(TASK_PATH);
                            }
                            None => {println!("Invalid task returned");}
                        }
                    }
                    Err(_) => {println!("Invalid number - Cannot be parsed")}
                }
            }
        },
        Some("clear") => {
            let mut mode = ClearMode::Done;
            for arg in &args {
                match arg.as_str() {
                    "-all" | "-a" => {mode = ClearMode::All}
                    "-overdue" | "-od" => {mode = ClearMode::OverDue}
                    _ => {}
                }
            }

            match mode {
                ClearMode::All => {
                    tasks.clear_all();
                    tasks.save_to_json(TASK_PATH);
                },
                ClearMode::Done => {
                    tasks.clear_done();
                    tasks.save_to_json(TASK_PATH);
                },
                ClearMode::OverDue => {
                    tasks.clear_overdue();
                    tasks.save_to_json(TASK_PATH);
                }
            }
        },
        Some("delete") => {
            if args.is_empty() {
                println!("There should be at least 1 id")
            }
            let mut ids: Vec<u32> = Vec::new();
            for arg in &args {
                match arg.parse::<u32>() {
                    Ok(id) => {ids.push(id)},
                    Err(_) => {println!("Could not parse this argument. Invalid ID: {}", arg);}
                }
            }
            let mut counter: u32 = 0;

            if !ids.is_empty() {
                for id in ids {
                    match tasks.get_task(id) {
                        Some(_) => {tasks.delete_task(id); counter += 1;}
                        None => {println!("Invalid ID: {}", id)}
                    }
                }
            }
            println!("Deleted {} task(s)", counter);
            tasks.next_id();
            tasks.save_to_json(TASK_PATH);
        }
        Some(_other) => {
            display_commands();
        },
        None => {
            println!("Error");
        }
    }
}

fn display_commands() {
    println!("\n=== AVAILABLE COMMANDS ===\n");

    println!("help");
    println!("  → Show all available commands\n");

    println!("list | ls");
    println!("  → List all tasks");
    println!("  Flags:");
    println!("    -e | -ext | -extended   → Show extended view (includes tags)\n");

    println!("add");
    println!("  → Add a new task");
    println!("  Usage:");
    println!("    add -t <title> -tg <tag1> <tag2> ...\n");

    println!("done");
    println!("  → Mark task(s) as completed");
    println!("  Usage:");
    println!("    done <id>");
    println!("    done <id1> <id2> ...\n");

    println!("exit");
    println!("  → Exit the program\n");

    println!("==============================\n");
}



//Ratatui 
/*
Needs an app struct with an exit flag
Run the loop with the app struct
Display Tasks for now and exit with a letter
*/
// MAIN INPUT LOOP
fn main() -> io::Result<()> {
    //input loop
    let mut tasks = TaskList::load(TASK_PATH);

    // tasks.list();
    // loop {
    //     take_input(&mut tasks);
    // }
    // color_eyre::install()?;
    let mut terminal = ratatui::init();
    let mut app = App { exit: false };
    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result

}

pub struct  App {
    exit: bool,
}
impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            match crossterm::event::read()? {
                crossterm::event::Event::Key(key_event) => self.handle_key_event(key_event)?,
                _ => {}
            }
            terminal.draw(|frame| self.draw(frame))?;
        }
        Ok(())
    }
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
    fn handle_key_event(&mut self, key_event: KeyEvent) -> io::Result<()> {
        if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Char('q') {
            self.exit = true;
        }
        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
        where
            Self: Sized {
        Line::from("Tasks Overview").centered().bold().render(area, buf);
    }
}