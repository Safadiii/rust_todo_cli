use chrono::Local;
use humantime::format_duration;
use ratatui::{Frame, layout::{Constraint, Rect}, style::{Color, Style}, widgets::{Block, BorderType, Borders, Clear, Paragraph}};

use crate::app::App;

impl App {
    pub fn render_details(&mut self, frame: &mut Frame, area: Rect) {
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
}