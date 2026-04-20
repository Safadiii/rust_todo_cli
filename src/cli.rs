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

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
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