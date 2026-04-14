use std::fs;
use std::io;
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
use ratatui::widgets::Block;
use ratatui::widgets::Borders;
use ratatui::widgets::Clear;
use ratatui::widgets::List;
use ratatui::widgets::ListItem;
use ratatui::widgets::ListState;
use ratatui::widgets::Paragraph;
use ratatui::widgets::TitlePosition;
use serde::de;
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


//Ratatui 
/*
Needs an app struct with an exit flag
Run the loop with the app struct
Display Tasks for now and exit with a letter
*/
// MAIN INPUT LOOP
fn main() -> Result<()> {
    //input loop
    let mut tasks = TaskList::load(TASK_PATH);
    let due = parse_due("2h");

    tasks.add("Helping People", vec!["Eating food".to_string()], due);


    // tasks.list();
    // loop {
    //     take_input(&mut tasks);
    // }
    color_eyre::install()?;
    ratatui::run(|terminal  | App::new(tasks).run(terminal))
}

#[derive(Debug, Clone, Copy)] // Add Clone and Copy here

enum AddTaskState {
    InputMode,
    EditingMode,
    None
}
enum State {
    AddingTask,
    EditingTask,
    None
}

pub struct App {
    exit: bool,
    taskslist: TaskList,
    list_state: ListState,
    state: State,
}
impl App {
    fn new(tasks_list: TaskList) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            exit: false,
            taskslist: tasks_list,
            list_state,
            state: State::None,
        }
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
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match self.state {  
            State::None => {self.handle_normal(key_event)?},
            State::AddingTask => {self.handle_adding_task(key_event)?},
            _ => {}
        }
        Ok(())
    }
    fn handle_normal(&mut self, key_event: KeyEvent) -> Result<()> {
        if key_event.kind != KeyEventKind::Press {
            return Ok(());
        }
        match key_event.code {
            KeyCode::Char('q') => self.exit = true,

            KeyCode::Down | KeyCode::Char('j') => {
                let i = match self.list_state.selected() {
                    Some(i) => i + 1,
                    None => 0,
                };

                if i < self.taskslist.tasks.len() {
                    self.list_state.select(Some(i));
                }
            }

            KeyCode::Up | KeyCode::Char('k') => {
                let i = match self.list_state.selected() {
                    Some(i) => i.saturating_sub(1),
                    None => 0,
                };
                self.list_state.select(Some(i));
            }

            KeyCode::Char('a') => {
                match self.state {
                    State::None => {
                        self.state = State::AddingTask;
                    }
                    State::AddingTask => {
                        self.state = State::None;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        Ok(())
    }
    fn handle_adding_task(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Char('q') => {
                self.exit = true;
            }
            _ => {}
        }

        Ok(())
    }

    fn render_add_task_popup(&mut self, frame: &mut Frame, area: Rect) {
        let add_task_block = Block::bordered().title("Add Task").fg(Color::LightYellow);
        let centered_area = area.centered(Constraint::Percentage(60), Constraint::Percentage(60));

        let layout = Layout::vertical([
            Constraint::Percentage(80),
            Constraint::Percentage(20),]
        ).margin(1);
        
        let details_layout = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Length(3),
        ]).margin(1);

        let [details_area, footer_area] = centered_area.layout(&layout);

        let [task_title_area, task_tags_area, task_due_area] = details_area.layout(&details_layout);

        let help_vec = vec!["Press ".into(), "q".bold(), " to exit".into()];
        
        let help_text = Text::from(Line::from(help_vec).patch_style(Style::default()));

        let help_msg = Paragraph::new(help_text);

        let title_block = Block::default().title("Title").borders(Borders::ALL);
        let tags_block = Block::default().title("Tags").borders(Borders::ALL);
        let due_block = Block::default().title("Due").borders(Borders::ALL);


        let title = Paragraph::new("Hello").block(title_block);


        frame.render_widget(Clear, centered_area);

        frame.render_widget(add_task_block, centered_area);

        frame.render_widget(help_msg, footer_area);
        frame.render_widget(title, task_title_area);
        frame.render_widget(due_block, task_due_area);
        frame.render_widget(tags_block, task_tags_area);


    }

    fn render_tasks_block(&mut self, frame: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self.taskslist.tasks
                    .iter()
                    .map(|task| {
                        let (status, style) = match task.status {
                            Status::Done => ("[DONE]", Style::default().fg(Color::White)),
                            Status::InProgress => ("[IN PROGRESS]", Style::default().fg(Color::White)),
                        };

                        let content = format!("{}. {} {}", task.id, task.title, status);

                        ListItem::new(content).style(style)
                    })
                    .collect();
        let list = List::new(items)
                    .block(
                        Block::default()
                        .borders(Borders::ALL)
                        .title("Tasks").style(Color::White)
                    ).highlight_style(Modifier::REVERSED).highlight_symbol(">> ");
        frame.render_stateful_widget(list, area, &mut self.list_state);
    }

    fn render_details(&self, frame: &mut Frame, area: Rect) {
        let selected = self.list_state.selected();

        let content = if let Some(i) = selected {
            if let Some(task) = self.taskslist.tasks.get(i) {
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
            .title("Details");

        frame.render_widget(Paragraph::new(content).block(block), area);
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        let controls = vec![
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
            ];

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
                    .constraints([
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                    ]).split(vertical[0]);
        
        self.render_tasks_block(frame, top_chunks[0]);
        self.render_details(frame, top_chunks[1]);
        self.render_footer(frame, vertical[1]);
        match self.state {
            State::AddingTask => {
                self.render_add_task_popup(frame, area);
            }
            _ => {}
        }
    }
}

