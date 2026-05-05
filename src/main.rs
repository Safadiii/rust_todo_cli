// external crates
use chrono::{Local};
use color_eyre::Result;

mod task;
mod storage;
mod category;
mod app;
mod ui;
mod events;
mod search;
mod config;

use storage::{load};

use crate::{app::App, config::config::load_config};

const TASK_PATH: &str = "tasks.json";

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
    let config = load_config();
    color_eyre::install()?;
    ratatui::run(|terminal  | App::new(categories, config).run(terminal))
}
