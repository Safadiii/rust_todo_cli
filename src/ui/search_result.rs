use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, List, ListItem},
};

use crate::{App, app::MainFocus, search::SearchConfig, task::Status};
use crate::search::search_fuzzy;

impl App {
    pub fn render_results(&mut self, frame: &mut Frame, area: Rect) {
        let config = SearchConfig::default();
        let results = search_fuzzy(&self.categories, &self.cmd, &config);
        let active = self.config.active;
        let background = self.config.background;

        let is_focused = matches!(self.mainfocus, MainFocus::SearchResults);
        let border_color = if is_focused {
            self.config.active
        } else {
            self.config.inactive
        };

        let items: Vec<ListItem> = results
            .iter()
            .map(|result| {
                let status_symbol = match result.task.status {
                    Status::Done    => ("✓", Color::Green),
                    Status::InProgress => ("◉", Color::Yellow),
                    _ => ("○", Color::Gray),
                };

                // Line 1: status + task title + enter hint
                let line1 = Line::from(vec![
                    Span::styled(
                        format!("{} ", status_symbol.0),
                        Style::default().fg(status_symbol.1),
                    ),
                    Span::styled(
                        result.task.title.clone(),
                        Style::default()
                            .fg(active)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        "  ⏎ open",
                        Style::default()
                            .fg(active)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]);

                // Line 2: category + tags
                let mut line2_spans = vec![
                    Span::styled(
                        "   in ",
                        Style::default().fg(active),
                    ),
                    Span::styled(
                        result.category_title,
                        Style::default().fg(active),
                    ),
                ];

                if !result.task.tags.is_empty() {
                    line2_spans.push(Span::styled(
                        "  ·  ",
                        Style::default().fg(active),
                    ));
                    for tag in &result.task.tags {
                        line2_spans.push(Span::styled(
                            format!("#{} ", tag),
                            Style::default().fg(Color::Indexed(140)),
                        ));
                    }
                }

                let line2 = Line::from(line2_spans);

                // Spacer line between results
                let spacer = Line::from("");

                ListItem::new(Text::from(vec![line1, line2, spacer]))
            })
            .collect();

        let title = Line::from(vec![
            Span::raw(" Results "),
            Span::styled(
                format!("({}) ", results.len()),
                Style::default().fg(active),
            ),
        ]);

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick)
                    .border_style(Style::default().fg(border_color))
                    .title(title).bg(background),
            )
            .highlight_style(
                Style::default()
                    .bg(active)
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Black),
            )
            .highlight_symbol("  ");

        frame.render_stateful_widget(list, area, &mut self.searchliststate);
    }
}