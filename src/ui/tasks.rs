use crate::App;
use crate::app::MainFocus;
use ratatui::text::{Line, Span};
use ratatui::{Frame, style::Modifier};
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem};
use ratatui::layout::Rect;
use crate::task::{Status, Task};
use ratatui::style::{Color, Style};
use crate::format_short_duration;


impl App {
    pub fn render_tasks_block(&mut self, frame: &mut Frame, area: Rect) {
        let cat_idx = self.categoryliststate.selected().unwrap_or(0);
        let tasks = self.visible_tasks(cat_idx);

        let color = match self.mainfocus {
            MainFocus::Task => self.config.active,
            _ => self.config.inactive,
        };
        let items: Vec<ListItem> = tasks
            .iter()
            .map(|task| {
                let (symbol, status_color) = match task.status {
                    Status::Done => ("◆", self.config.active),
                    Status::InProgress => ("◇", self.config.active),
                };

                let left = format!("{} {}", symbol, task.title);

                let due_text = if let Some(due) = task.due {
                    let now = chrono::Local::now();
                    if due > now {
                        let d = (due - now).to_std().unwrap_or_default();
                        format!("in {}", format_short_duration(d))
                    } else {
                        let d = (now - due).to_std().unwrap_or_default();
                        format!("{} ago", format_short_duration(d))
                    }
                } else {
                    String::new()
                };

                let total_width = area.width as usize;
                let spacing = total_width
                    .saturating_sub(left.len() + due_text.len() + 1);

                let spaces = " ".repeat(spacing);

                let line = Line::from(vec![
                    Span::styled(symbol, Style::default().fg(status_color)),
                    Span::raw(" "),
                    Span::raw(task.title.clone()),
                    Span::raw(spaces),
                    Span::styled(due_text, Style::default().fg(Color::DarkGray)),
                ]);

                ListItem::new(line)
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick)
                    .border_style(Style::default().fg(color))
                    .title("Tasks")
                    .style(Style::default().bg(self.config.background)),
            )
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Black)
                    .bg(color),
            );

        frame.render_stateful_widget(list, area, &mut self.list_state);
    }
}