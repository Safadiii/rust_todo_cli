use std::fs;
use std::fmt;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::time::Duration;
use color_eyre::Result;
use chrono::{DateTime, Local, Duration as ChronoDuration};
use color_eyre::eyre::Ok;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyEventKind;
use humantime::format_duration;
use ratatui::DefaultTerminal;
use ratatui::symbols::merge;
use ratatui::text::Text;
use ratatui::layout::Alignment;
use ratatui::layout::Constraint;
use ratatui::layout::Direction;
use ratatui::layout::Layout;
use ratatui::style::Color;
use ratatui::style::Modifier;
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::text::Span;
use ratatui::symbols::merge::MergeStrategy;
use ratatui::layout::Spacing;
use ratatui::widgets::Block;
use ratatui::widgets::BorderType;
use ratatui::widgets::Borders;
use ratatui::widgets::Clear;
use ratatui::widgets::List;
use ratatui::widgets::ListItem;
use ratatui::widgets::ListState;
use ratatui::widgets::Padding;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Wrap;
use serde::{Serialize, Deserialize};
use humantime::parse_duration;
use ratatui::{Frame, style::Style, layout::Rect};

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
    fn new() -> Self {
        let tasks: Vec<Task> = vec![];
        Self {
            tasks: tasks,
            next_id: 0,
        }
    }
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


// fn take_input(tasks: &mut TaskList) {

//     //Input from the CLI
//     let mut input: String = String::new();

//     io::stdin().read_line(&mut input).expect("Could not read line");

//     let mut parts = input.split_whitespace();

//     let cmd = parts.next();
//     let args: Vec<String> = parts.map(str::to_owned).collect();

//     match cmd {
//         Some("add") => {
//             //initialize the empty vector strings
//             let mut title_parts: Vec<String> = Vec::new();
//             let mut tag_parts: Vec<String> = Vec::new();

//             let mut mode = AddMode::None;

//             let mut due: Option<DateTime<Local>> = None;

//             for arg in &args {
//                 match arg.as_str() {
//                     "-t" | "-title" => mode = AddMode::Title,
//                     "-tg" | "-tags" => mode = AddMode::Tags,
//                     "-due" => mode = AddMode::Due,
//                     _ => match mode {
//                         AddMode::Title => title_parts.push(arg.clone()),
//                         AddMode::Tags => tag_parts.push(arg.clone()),
//                         AddMode::Due => {
//                             match parse_duration(arg) {
//                                 Ok(duration) => {
//                                     let now = Local::now();
//                                     due = Some(now + chrono::Duration::from_std(duration).unwrap());
//                                 }
//                                 Err(_) => {
//                                     println!("Error parsing time.")
//                                 }
//                             }
//                         }
//                         AddMode::None => {}
//                     }
//                 }
//             }

//             let title = title_parts.join(" ");

//             if !title.is_empty() {
//                 tasks.add(&title, tag_parts, due);
//                 tasks.save_to_json(TASK_PATH);
//             } else {
//                 println!("Error add a title using -t flag");
//             }
//         }

//         Some("help") => {
//             display_commands();
//         },
//         Some("list") | Some("ls") => {
//             let mut mode = ListMode::Standard;
//             for arg in &args {
//                 match arg.as_str() {
//                     "-ext" | "-e" | "-extended" => mode = ListMode::Extended,
//                     "-sort" | "-sorted" => mode = ListMode::Sorted,
//                     _ => mode = ListMode::Standard,
//                 }
//             }
//             match mode {
//                 ListMode::Extended => tasks.list_extended(),
//                 ListMode::Standard => tasks.list(),
//                 ListMode::Sorted => {tasks.sort_by_deadline(); tasks.list();}
//             }
//         },
//         Some("exit") => {
//             std::process::exit(23);
//         },
//         Some("done") => {
//             if let [first] = args.as_slice() {
//                 match first.parse::<u32>() {
//                     Ok(num) => {
//                         match tasks.get_task(num) {
//                             Some(task) => {
//                                 task.mark_completed();
//                                 tasks.save_to_json(TASK_PATH);
//                             }
//                             None => {println!("Invalid task returned");}
//                         }
//                     }
//                     Err(_) => {println!("Invalid number - Cannot be parsed")}
//                 }
//             }
//         },
//         Some("clear") => {
//             let mut mode = ClearMode::Done;
//             for arg in &args {
//                 match arg.as_str() {
//                     "-all" | "-a" => {mode = ClearMode::All}
//                     "-overdue" | "-od" => {mode = ClearMode::OverDue}
//                     _ => {}
//                 }
//             }

//             match mode {
//                 ClearMode::All => {
//                     tasks.clear_all();
//                     tasks.save_to_json(TASK_PATH);
//                 },
//                 ClearMode::Done => {
//                     tasks.clear_done();
//                     tasks.save_to_json(TASK_PATH);
//                 },
//                 ClearMode::OverDue => {
//                     tasks.clear_overdue();
//                     tasks.save_to_json(TASK_PATH);
//                 }
//             }
//         },
//         Some("delete") => {
//             if args.is_empty() {
//                 println!("There should be at least 1 id")
//             }
//             let mut ids: Vec<u32> = Vec::new();
//             for arg in &args {
//                 match arg.parse::<u32>() {
//                     Ok(id) => {ids.push(id)},
//                     Err(_) => {println!("Could not parse this argument. Invalid ID: {}", arg);}
//                 }
//             }
//             let mut counter: u32 = 0;

//             if !ids.is_empty() {
//                 for id in ids {
//                     match tasks.get_task(id) {
//                         Some(_) => {tasks.delete_task(id); counter += 1;}
//                         None => {println!("Invalid ID: {}", id)}
//                     }
//                 }
//             }
//             println!("Deleted {} task(s)", counter);
//             tasks.next_id();
//             tasks.save_to_json(TASK_PATH);
//         }
//         Some(_other) => {
//             display_commands();
//         },
//         None => {
//             println!("Error");
//         }
//     }
// }

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


fn parse_due(input: &str) -> Option<DateTime<Local>> {
    let duration = parse_duration(input).ok()?;
    let chrono_duration = chrono::Duration::from_std(duration).ok()?;
    Some(Local::now() + chrono_duration)
}

/*

Helper Function

*/


//Changing char to bytes
fn char_to_byte_index(s: &str, char_index: usize) -> usize {
    s.char_indices()
        .nth(char_index)
        .map(|(i, _)| i)
        .unwrap_or(s.len())
}   

fn due_parse(s: String) -> bool {
    humantime::parse_duration(s.as_str()).is_ok()
}

fn format_short_duration(duration: std::time::Duration) -> String {
    let total_secs = duration.as_secs();

    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;

    match (hours, minutes) {
        (h, m) if h > 0 && m > 0 => format!("{}h {}m", h, m),
        (h, _) if h > 0 => format!("{}h", h),
        (_, m) => format!("{}m", m),
    }
}


//Ratatui 
/*
Needs an app struct with an exit flag
Run the loop with the app struct
Display Tasks for now and exit with a letter
*/
// MAIN INPUT LOOP
fn main() -> Result<()> {
    //input loop

    // tasks.list();
    // loop {
    //     take_input(&mut tasks);
    // }
    let categories = load(TASK_PATH);
    color_eyre::install()?;
    ratatui::run(|terminal  | App::new(categories).run(terminal))
}

#[derive(Debug, Clone, Copy)] // Add Clone and Copy here

enum CmdMode {
    AddingCategory,
    None
}
enum Focus {
    None,
    AddTaskPopup,
    DetailsPopup,
    HelpPopup,
}

enum AddTaskField {
    Title,
    Tags,
    Due
}

enum MainFocus {
    Task,
    Categories,
    None
}
#[derive(Deserialize, Serialize)]
pub struct Category {
    title: String,
    taskslist: TaskList,
    parent: Option<Box<Category>>,
}
impl Category {
    fn new(title: String, parent: Option<Box<Category>>) -> Self {
        Self {
            title,
            taskslist: TaskList::new(),
            parent
        }
    }
    fn add_task(&mut self, task: Task) {
        self.taskslist.tasks.push(task);
    }
}

fn save(path: &str, categories: &Vec<Category>) {
    let json = serde_json::to_string_pretty(categories)
        .expect("Could not serialize categories.");

    fs::write(path, json)
        .expect("Could not write to file.");
}


fn load(path: &str) -> Vec<Category> {
        if !Path::new(path).exists() {
            let categories: Vec<Category> = vec![];

            return categories;
        } 
        let data = fs::read_to_string(path).unwrap_or_else(|_| {
            println!("Failed to create file, creating new one.");
            return String::new();
        });

        if data.trim().is_empty() {
            return vec![];
        }

        serde_json::from_str(&data).unwrap_or_else(|_| {
            println!("Corrupted file, couldn't read.");
            vec![]
        })
    }
pub struct App {
    exit: bool,
    taskslist: TaskList,
    list_state: ListState,
    focus: Focus,
    addtaskfield: AddTaskField,  //Selected field in add task popup
    title_input: String,
    tags_input: String,
    due_input: String,
    char_index: usize,
    inputtingMode: bool,
    mainfocus: MainFocus,
    categories: Vec<Category>,
    categoryliststate: ListState,
    cmd: String,
    cmd_index: usize,
    commandMode: CmdMode,
}
impl App {
    fn new(categories: Vec<Category>) -> Self {
        let mut list_state = ListState::default();
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
            char_index: 0,
            inputtingMode: false,
            mainfocus: MainFocus::Categories,
            categories,
            categoryliststate,
            cmd: String::new(),
            cmd_index: 0,
            commandMode: CmdMode::None,
        }
    }
    pub fn exit(&mut self) {
        self.exit = true;
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            match crossterm::event::read()? {
                crossterm::event::Event::Key(key_event) => self.handle_key_event(key_event)?,
                _ => {}
            }
        }
        Ok(())
    }
    

    //Get Current Task From Selected Category + List State
    fn current_task_mut(&mut self) -> Option<&mut Task> {
        self.categoryliststate
            .selected()
            .and_then(|cat_i| self.categories.get_mut(cat_i))
            .and_then(|cat| {
                self.list_state
                    .selected()
                    .and_then(|task_i| cat.taskslist.tasks.get_mut(task_i))
            })
    }


    //Cursor logic
    fn clamp_cursor(&mut self) {
        match self.focus {
            Focus::AddTaskPopup => {
                let len = match self.addtaskfield {
                    AddTaskField::Title => self.title_input.chars().count(),
                    AddTaskField::Tags => self.tags_input.chars().count(),
                    AddTaskField::Due => self.due_input.chars().count(),
                };
                self.char_index = self.char_index.clamp(0, len);
            }
            Focus::None => {
                self.cmd_index = self.cmd_index.clamp(0, self.cmd.chars().count())
            }
            _ => {}
        }

    }

    fn render_cursor(&self, frame: &mut Frame, areas: (Rect, Rect, Rect)) {
        let (title, tags, due) = areas;
        let (area, index) = match self.addtaskfield {
            AddTaskField::Title => (title, self.char_index),
            AddTaskField::Tags => (tags, self.char_index),
            AddTaskField::Due => (due, self.char_index),
        };

        let x = area.x + 1 + index as u16;
        let y = area.y + 1;

        frame.set_cursor_position((x, y));
    }
    
    fn move_cursor_to_end(&mut self) {
        match self.focus {
            Focus::AddTaskPopup => {
                self.char_index = match self.addtaskfield {
                    AddTaskField::Title => {self.title_input.chars().count()}
                    AddTaskField::Tags => {self.tags_input.chars().count()}
                    AddTaskField::Due => {self.due_input.chars().count()}
                };
            }
            _ => {}
        }

    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
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
            _ => {}
        }
        Ok(())
    }
    fn handle_helpkeys(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Esc => {
                self.focus = Focus::None;
            }
            _ => {}
        }
        Ok(())
    }
    fn handle_detailspopup(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Esc => {
                self.focus = Focus::None;
            }
            _ => {}
        }
        Ok(())
    }
    fn handle_cmd_events(&mut self, key_event: KeyEvent) -> Result<()> {
        if key_event.kind != KeyEventKind::Press {
            return Ok(())
        }
        match key_event.code {
            KeyCode::Esc => {
                self.cmd = String::new();
                self.cmd_index = 0;
                self.mainfocus = MainFocus::Categories;
                self.commandMode = CmdMode::None;
            }
            KeyCode::Char(c) => {
                self.cmd.push(c);
                self.cmd_index += 1;
            }
            KeyCode::Backspace => {
                if self.cmd_index > 0 {
                    self.cmd_index = self.cmd_index.saturating_sub(1);
                    let bytes = char_to_byte_index(self.cmd.as_str(), self.cmd_index);
                    self.cmd.remove(bytes);
                }
            }
            KeyCode::Enter => {
                //Later change into a function to generalize
                match self.commandMode {
                    CmdMode::AddingCategory => {
                        self.categories.push(Category::new(std::mem::take(&mut self.cmd), None));
                        self.mainfocus = MainFocus::Categories;
                        self.commandMode = CmdMode::None;
                    }
                    _ => {}
                }
            }
            KeyCode::Right => {self.cmd_index += 1; self.clamp_cursor();}
            KeyCode::Left => {self.cmd_index = self.cmd_index.saturating_sub(1); self.clamp_cursor();}
            _ => {}
        }
        Ok(())
    }
    fn handle_main(&mut self, key_event: KeyEvent) -> Result<()> {
        if key_event.kind != KeyEventKind::Press {
            return Ok(());
        }
        match key_event.code {
            KeyCode::Char('q') => self.exit = true,

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
            KeyCode::Char('h') => {
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
                        self.commandMode = CmdMode::AddingCategory;
                        self.mainfocus = MainFocus::None;
                        self.clamp_cursor();
                    }
                    _ => {}
                }
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
                    _ => {}
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
    fn handle_popup(&mut self, key_event: KeyEvent) -> Result<()> {
        if !self.inputtingMode {
                match key_event.code {
                    KeyCode::Tab => {
                        match self.addtaskfield {
                            AddTaskField::Title => {self.addtaskfield = AddTaskField::Tags; self.move_cursor_to_end();},
                            AddTaskField::Due => {self.addtaskfield = AddTaskField::Title; self.move_cursor_to_end();},
                            AddTaskField::Tags => {self.addtaskfield = AddTaskField::Due; self.move_cursor_to_end();},
                        }
                        self.clamp_cursor();
                    }
                    KeyCode::Char('j') => {
                        match self.addtaskfield {
                            AddTaskField::Title => {self.addtaskfield = AddTaskField::Tags; self.move_cursor_to_end();},
                            AddTaskField::Due => {self.addtaskfield = AddTaskField::Title; self.move_cursor_to_end();},
                            AddTaskField::Tags => {self.addtaskfield = AddTaskField::Due; self.move_cursor_to_end();},
                        }
                        self.clamp_cursor();
                    }
                    KeyCode::Char('k') => {
                        match self.addtaskfield {
                            AddTaskField::Title => {self.addtaskfield = AddTaskField::Due; self.move_cursor_to_end();},
                            AddTaskField::Due => {self.addtaskfield = AddTaskField::Tags; self.move_cursor_to_end();},
                            AddTaskField::Tags => {self.addtaskfield = AddTaskField::Title; self.move_cursor_to_end();},
                        }
                        self.clamp_cursor();
                    }
                    KeyCode::Char('c') => {
                        match self.addtaskfield {
                            AddTaskField::Title => {self.title_input.clear(); self.clamp_cursor();}
                            AddTaskField::Tags => {self.tags_input.clear(); self.clamp_cursor();}
                            AddTaskField::Due => {self.due_input.clear(); self.clamp_cursor();}
                        }
                    }
                    KeyCode::Char('q') | KeyCode::Esc => {self.focus = Focus::None},
                    KeyCode::Char('e') | KeyCode::Char('i') => {self.inputtingMode = true; self.move_cursor_to_end();}
                    KeyCode::Right => {self.char_index += 1; self.clamp_cursor();}
                    KeyCode::Left => {self.char_index = self.char_index.saturating_sub(1); self.clamp_cursor();}
                    KeyCode::Enter => {
                        let due = &mut self.due_input;
                        if !self.title_input.is_empty() && !self.tags_input.is_empty() && due_parse(due.to_string()) {
                            let now = Local::now();
                            let duration = parse_duration(due).unwrap();
                            let taskdue = Some(now + chrono::Duration::from_std(duration).unwrap());

                            let tags: Vec<String> = self.tags_input
                                .split_whitespace()
                                .map(|x| x.to_string())
                                .collect();
                            let _curr_category = match self.categoryliststate.selected().and_then(|i| self.categories.get_mut(i)) {
                                Some(category) => {
                                   category.taskslist.add(self.title_input.as_str(), tags, taskdue);
                                   save(TASK_PATH, &self.categories);
                                }
                                _ => {}
                        };
                        }
                        self.title_input = String::new();
                        self.tags_input = String::new();
                        self.due_input = String::new();
                        self.focus = Focus::None;
                        self.addtaskfield = AddTaskField::Title;
                    }
                    _ => {}
            }
        } else {
            match key_event.code {
                KeyCode::Esc => {self.inputtingMode = false;},
                KeyCode::Char(c) => {
                    let input = match self.addtaskfield {
                        AddTaskField::Title => {&mut self.title_input},
                        AddTaskField::Tags => {&mut self.tags_input},
                        AddTaskField::Due => {&mut self.due_input},
                    };

                    let byte_index = char_to_byte_index(input, self.char_index);
                    input.insert(byte_index, c);

                    self.char_index += 1;
                }
                KeyCode::Backspace => {
                    let input = match self.addtaskfield {
                        AddTaskField::Title => {&mut self.title_input},
                        AddTaskField::Tags => {&mut self.tags_input},
                        AddTaskField::Due => {&mut self.due_input},
                    };
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
                        AddTaskField::Title => {self.addtaskfield = AddTaskField::Tags; self.clamp_cursor(); self.move_cursor_to_end();}
                        AddTaskField::Tags => {self.addtaskfield = AddTaskField::Due; self.clamp_cursor(); self.move_cursor_to_end();}
                        AddTaskField::Due => {self.addtaskfield = AddTaskField::Title; self.clamp_cursor(); self.move_cursor_to_end();}
                    } 
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn render_add_task_popup(&mut self, frame: &mut Frame, area: Rect) {
        self.clamp_cursor();
        let color_main = Color::Indexed(73);
        let mode_span = if self.inputtingMode {
            Span::styled(" INSERT ", Style::default().bg(Color::White).fg(Color::Indexed(73)))
        } else {
            Span::styled(" NORMAL ", Style::default().bg(Color::Indexed(73)).fg(Color::White))
        };

        let title = Line::from(vec![
            Span::raw("Add Task "),
            mode_span,
        ]);

        let add_task_block = Block::bordered()
            .title(title)
            .fg(color_main);
        let centered_area = area.centered(Constraint::Percentage(50), Constraint::Max(15));

        let popup_height = 13;


        let popup_area = centered_area.centered(Constraint::Fill(1), Constraint::Length(popup_height as u16));

        let block = Block::default().padding(Padding::horizontal(1)).inner(popup_area);

        
        let details_layout = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ]);


        let [task_title_area, task_tags_area, task_due_area] = block.layout(&details_layout);

        let title_block = Block::default().title("Title").borders(Borders::ALL).style(
            match self.addtaskfield {
                AddTaskField::Title => Style::default().fg(color_main),
                _ => Style::default().fg(Color::Indexed(250)),
            }
        );
        let tags_block = Block::default().title("Tags").borders(Borders::ALL).style(
            match self.addtaskfield {
                AddTaskField::Tags => Style::default().fg(color_main),
                _ => Style::default().fg(Color::Indexed(250)),
            }
        );
        let due_block = Block::default().title("Due").borders(Borders::ALL).style(
            match self.addtaskfield {
                AddTaskField::Due => Style::default().fg(color_main),
                _ => Style::default().fg(Color::Indexed(250)),
            }
        );


        let text = Text::from(self.title_input.as_str());
        let tags_text = Text::from(self.tags_input.as_str());
        let due_text = Text::from(self.due_input.as_str());


        let title = Paragraph::new(text).block(title_block).wrap(Wrap { trim: false});
        let tags = Paragraph::new(tags_text).block(tags_block);
        let due = Paragraph::new(due_text).block(due_block);


        frame.render_widget(Clear, centered_area);

        frame.render_widget(add_task_block, centered_area);

        frame.render_widget(title, task_title_area);
        frame.render_widget(due, task_due_area);
        frame.render_widget(tags, task_tags_area);

        self.render_cursor(frame, (task_title_area, task_tags_area, task_due_area));
    }

    fn render_tasks_block(&mut self, frame: &mut Frame, area: Rect) {
        let tasks = if let Some(i) = self.categoryliststate.selected() {
            if let Some(category) = self.categories.get(i) {
                &category.taskslist.tasks
            } else {
                &self.taskslist.tasks
            }
        } else {
            &self.taskslist.tasks
        };


        let items: Vec<ListItem> = tasks
            .iter()
            .map(|task| {
                let (symbol, color) = match task.status {
                    Status::Done => ("◆", Color::Indexed(73)),
                    Status::InProgress => ("◇", Color::Indexed(73)),
                };

                let left = format!("{} {}", symbol, task.title);

                let due_text = if let Some(due) = task.due {
                    let now = Local::now();
                    let text = if due > now {
                        let d = (due - now).to_std().unwrap_or_default();
                        format!("in {}", format_short_duration(d))
                    } else {
                        let d = (now - due).to_std().unwrap_or_default();
                        format!("{} ago", format_short_duration(d))
                    };
                    text
                } else {
                    String::new()
                };

                // padding calculation
                let total_width = area.width as usize;
                let left_len = left.len();
                let right_len = due_text.len();

                let spacing = total_width.saturating_sub(left_len + right_len + 1);
                let spaces = " ".repeat(spacing);

                let line = Line::from(vec![
                    Span::styled(format!("{}", symbol), Style::default().fg(color)),
                    Span::raw(" "),
                    Span::raw(task.title.clone()),
                    Span::raw(spaces),
                    Span::styled(due_text, Style::default().fg(Color::DarkGray)),
                ]);

                ListItem::new(line)
            })
            .collect();
        let color = match self.mainfocus {
            MainFocus::Task => Color::Indexed(73),
            _ => Color::Indexed(250),
        };
        let list = List::new(items)
                    .block(
                        Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Thick)
                        .border_style(Style::default().fg(color))
                        .merge_borders(MergeStrategy::Exact)
                        .title("Tasks").style(Style::default().bg(Color::Indexed(240)))
                    ).highlight_style(Style::default().add_modifier(Modifier::BOLD).bg(Color::Indexed(73)));
        frame.render_stateful_widget(list, area, &mut self.list_state);
    }

    fn render_details(&mut self, frame: &mut Frame, area: Rect) {
        let _curr_category = match self.categoryliststate.selected().and_then(|i| self.categories.get_mut(i)) {
            Some(category) => {
                let popup_area = area.centered(
                    Constraint::Percentage(60),
                    Constraint::Percentage(60),
                );
                let selected = self.list_state.selected();
                let content = if let Some(i) = selected {
                if let Some(task) = category.taskslist.tasks.get(i) {
                    let tags = if task.tags.is_empty() {
                        "None".to_string()
                    } else {
                        task.tags
                            .iter()
                            .map(|t| format!("- {}", t))
                            .collect::<Vec<_>>()
                            .join("\n")
                    };

                    let now = Local::now();

                    let due_text = if let Some(due) = task.due {
                        match (due - now).to_std() {
                            std::result::Result::Ok(dur) => format!("Due: {}", format_duration(dur)),
                            Err(_) => format!("Overdue"),
                        }
                    } else {
                        "No due date".to_string()
                    };
                    format!(
                        "Title: {}\n\nStatus: {:?}\n\nTags:\n{} \n\n{}",
                        task.title, task.status, tags, due_text
                    )
                } else {
                    "No task selected".to_string()
                }
            } else {
                "No task selected".to_string()
            };
            let block = Block::default()
                            .borders(Borders::ALL)
                            .border_type(BorderType::Thick)
                            .border_style(Style::default().fg(Color::Indexed(73)))
                            .title("Details").style(Style::default().bg(Color::Indexed(240)));
            frame.render_widget(Clear, popup_area);
            frame.render_widget(Paragraph::new(content).block(block), popup_area);
            }
            _ => {}
        };


    }

    fn render_categories(&mut self, frame: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self.categories
                    .iter()
                    .map(|category| {

                        let content = format!("{}", category.title);

                        ListItem::new(content)
                    })
                    .collect();
        let color: Color = match self.mainfocus {
            MainFocus::Categories => Color::Indexed((73)),
            _ => Color::Indexed(250),
        };
        let list = List::new(items)
            .block(
                Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
                .border_style(Style::default().fg(color))
                .merge_borders(MergeStrategy::Exact)
                .title("Categories").style(Style::default().bg(Color::Indexed(240)))
            ).highlight_style(Style::default().add_modifier(Modifier::BOLD).bg(Color::Indexed(73))).highlight_symbol(">> ");
        frame.render_stateful_widget(list, area, &mut self.categoryliststate);
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        let controls: Vec<Line<>> = match self.focus {
            Focus::AddTaskPopup => {
                if self.inputtingMode {
                    vec![
                        Line::from(vec![
                            Span::styled("EDITING MODE:  ", Style::default().bg(Color::Reset).fg(Color::Red).add_modifier(Modifier::BOLD)),
                            Span::styled("ESC", Style::default().bg(Color::Reset).fg(Color::White).add_modifier(Modifier::UNDERLINED)),
                            Span::styled(" → STANDARD MODE   ", Style::default()),
                            Span::styled("ENTER", Style::default().add_modifier(Modifier::UNDERLINED)),
                            Span::raw(" → NEXT INPUT"),
                        ])
                    ]
                } else {
                    vec![
                        Line::from(vec![
                            Span::styled("STANDARD MODE:  ", Style::default().bg(Color::Reset).fg(Color::White).add_modifier(Modifier::BOLD)),
                            Span::styled("Q", Style::default().bg(Color::Reset).fg(Color::White).add_modifier(Modifier::UNDERLINED)),
                            Span::styled(" → CANCEL   ", Style::default()),
                            Span::styled("DOWN/UP", Style::default().add_modifier(Modifier::UNDERLINED)),
                            Span::raw(" → DOWN/UP   "),
                            Span::styled("E/I", Style::default().add_modifier(Modifier::UNDERLINED)),
                            Span::raw(" → EDIT FIELD   "),
                            Span::styled("TAB", Style::default().add_modifier(Modifier::UNDERLINED)),
                            Span::raw(" → NEXT FIELD   "),
                            Span::styled("ENTER", Style::default().add_modifier(Modifier::UNDERLINED)),
                            Span::raw(" → SUBMIT"),
                        ])
                    ]
                }
            }
            _ => {
                vec![
                    Line::from(vec![
                        Span::styled("q", Style::default().fg(Color::Red).add_modifier(Modifier::UNDERLINED)),
                        Span::raw(" → Quit   "),
                        Span::styled("↑/↓", Style::default().add_modifier(Modifier::UNDERLINED)),
                        Span::raw(" → Navigate   "),
                        Span::styled("j/k", Style::default().add_modifier(Modifier::UNDERLINED)),
                        Span::raw(" → Navigate   "),
                        Span::styled("d", Style::default().add_modifier(Modifier::UNDERLINED)),
                        Span::raw(" → Delete   "),
                        Span::styled("x", Style::default().add_modifier(Modifier::UNDERLINED)),
                        Span::raw(" → Done"),
                    ])
                ]
            }
        };


        let footer = Paragraph::new(controls)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(
                        Line::from(Span::styled(
                            " Controls ",
                            Style::default().add_modifier(Modifier::BOLD),
                        ))
                        .centered(),
                    ),
            );
        frame.render_widget(footer, area);
    }



    fn render_command_center(&mut self, frame: &mut Frame, area: Rect) {
        let color = match self.mainfocus {
            MainFocus::None => {
                Color::Indexed(73)
            }
            _ => Color::Indexed(250)
        };

        let title: String = match self.commandMode {
            CmdMode::AddingCategory => String::from("Category Name"),
            _ => String::new()
        };

        let cmd_input = Text::from(self.cmd.as_str());
        let block = Block::default().borders(Borders::ALL).border_style(Style::default().fg(color)).title(title);
        let inner = block.inner(area);
        let cmd = Paragraph::new(cmd_input).block(block);
        frame.render_widget(cmd, area);
        match self.mainfocus {
            MainFocus::None => {frame.set_cursor_position((inner.x + self.cmd.chars().count() as u16, inner.y));
}           _ => {}
        }
    }

    fn render_help_screen(&mut self, frame: &mut Frame, area: Rect) {
        let popup_area = area.centered(
            Constraint::Percentage(80),
            Constraint::Percentage(80),
        );

        let color = Color::Indexed(73);

        let help_text = vec![
            Line::from(Span::styled("Global", Style::default().add_modifier(Modifier::BOLD)).fg(color)),
            Line::from(" q        → Quit"),
            Line::from(" Tab      → Switch focus (Categories / Tasks)"),
            Line::from(" Esc      → Back / Exit popup"),
            Line::from(""),

            Line::from(Span::styled("Navigation", Style::default().add_modifier(Modifier::BOLD))),
            Line::from(" j / ↓    → Move down"),
            Line::from(" k / ↑    → Move up"),
            Line::from(" Enter    → Select / Open"),
            Line::from(""),

            Line::from(Span::styled("Tasks", Style::default().add_modifier(Modifier::BOLD))),
            Line::from(" a        → Open Add Task popup"),
            Line::from(" x        → Mark task completed"),
            Line::from(" p        → Mark task in progress"),
            Line::from(""),

            Line::from(Span::styled("Categories", Style::default().add_modifier(Modifier::BOLD))),
            Line::from(" C        → Add category (command mode)"),
            Line::from(""),

            Line::from(Span::styled("Command Mode", Style::default().add_modifier(Modifier::BOLD))),
            Line::from(" Type     → Enter command text"),
            Line::from(" Enter    → Confirm"),
            Line::from(" Esc      → Cancel"),
            Line::from(""),

            Line::from(Span::styled("Add Task Popup", Style::default().add_modifier(Modifier::BOLD))),
            Line::from(" Tab/j/k  → Switch fields"),
            Line::from(" e / i    → Enter input mode"),
            Line::from(" Esc      → Exit input mode"),
            Line::from(" c        → Clear field"),
            Line::from(" Enter    → Next field / Submit"),
            Line::from(" ← / →    → Move cursor"),
        ];

        let paragraph = Paragraph::new(help_text)
            .block(
                Block::default()
                    .title(" Help ")
                    .borders(Borders::ALL)
                    .fg(color)
            )
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        frame.render_widget(paragraph, popup_area);
    }
    
    fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();

        let vertical = Layout::default()
                            .direction(Direction::Vertical)
                            .constraints([
                                Constraint::Min(0),
                                Constraint::Length(3),]
                            ).split(area);

        let top_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .spacing(Spacing::Overlap(1))
                    .constraints([
                        Constraint::Percentage(30),
                        Constraint::Percentage(71),
                    ]).split(vertical[0]);

        match self.focus {
            Focus::HelpPopup => {self.render_help_screen(frame, area);
            return}
            _ => {}
        }
        
        self.render_tasks_block(frame, top_chunks[1]);
        self.render_categories(frame, top_chunks[0]);
        // self.render_footer(frame, vertical[1]);
        self.render_command_center(frame, vertical[1]);
       match self.focus {
            Focus::AddTaskPopup => {
                frame.render_widget(
                    Block::default().style(Style::default().bg(Color::Black)),
                    area,
                );
                self.render_add_task_popup(frame, area);
            },
            Focus::DetailsPopup => {
                self.render_details(frame, area);
            }
            _ => {}
        }
    }
}

