use std::io;
use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
enum Status {
    Done, InProgress 
}
enum AddMode {
    None,
    Title,
    Tags
}
enum ListMode {
    Extended,
    Standard
}
enum ClearMode {
    All,
    Done
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
}

impl Task {
    fn create(id: u32, title: &str, tags: Vec<String>) -> Self {
        Self {
            id, title: title.to_string(), tags, status: Status::InProgress
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
    fn init() -> Self {
        Self {
            tasks: vec![],
            next_id: 1
        }
    }
    fn add(&mut self, title: &str, tags: Vec<String>) {
        let id = self.next_id;
        self.next_id += 1;
        self.tasks.push(Task::create(id, title, tags));
    }
    fn list(&self) {
        if self.tasks.is_empty() {
            println!("No tasks to show add a task by running add")
        } else {
            for task in &self.tasks {
                let id = &task.id;
                let title = &task.title;
                let progress = &task.status;
                println!("{id}- {title} {progress} ")
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
    }
}

fn main() {

    //input loop
    let mut tasks = TaskList::init();
    let title: &str = "Hello";
    let tags: Vec<String> = vec![
        "rust".to_string(),
        "cli".to_string(),
        "todo".to_string(),
    ];

    tasks.add(title, tags);

    tasks.list();
    loop {
        take_input(&mut tasks);
    }
}


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

            for arg in &args {
                match arg.as_str() {
                    "-t" | "-title" => mode = AddMode::Title,
                    "-tg" | "-tags" => mode = AddMode::Tags,
                    _ => match mode {
                        AddMode::Title => title_parts.push(arg.clone()),
                        AddMode::Tags => tag_parts.push(arg.clone()),
                        AddMode::None => {}
                    }
                }
            }

            let title = title_parts.join(" ");

            if !title.is_empty() {
                tasks.add(&title, tag_parts);
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
                    _ => mode = ListMode::Standard,

                }
            }
            match mode {
                ListMode::Extended => tasks.list_extended(),
                ListMode::Standard => tasks.list(),
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
                    _ => {}
                }
            }

            match mode {
                ClearMode::All => {tasks.clear_all();},
                ClearMode::Done => {tasks.clear_done();}
            }
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