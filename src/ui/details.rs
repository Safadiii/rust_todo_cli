use chrono::Local;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
};

use crate::{app::App, format_short_duration};

//claude generated TUI design for task popup

impl App {
    pub fn render_details(&mut self, frame: &mut Frame, area: Rect) {
        let active = self.config.active;
        let bg = self.config.background;
        let _curr_category = match self
            .categoryliststate
            .selected()
            .and_then(|i| self.categories.get_mut(i))
        {
            Some(category) => {
                let popup_area = area.centered(
                    Constraint::Percentage(55),
                    Constraint::Percentage(65),
                );

                let selected = self.list_state.selected();
                let cat_idx = self.categoryliststate.selected().unwrap_or(0);
                let tasks = self.visible_tasks(cat_idx);

                if let Some(i) = selected {
                    if let Some(task) = tasks.get(i) {
                        let outer_block = Block::default()
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded)
                            .border_style(Style::default().fg(active))
                            .title(Line::from(vec![
                                Span::raw(" "),
                                Span::styled(
                                    "Task Details",
                                    Style::default()
                                        .fg(active)
                                        .add_modifier(Modifier::BOLD),
                                ),
                                Span::raw(" "),
                            ]))
                            .title_alignment(Alignment::Center)
                            .style(Style::default().bg(bg));

                        frame.render_widget(Clear, popup_area);
                        frame.render_widget(&outer_block, popup_area);

                        // ── inner layout ─────────────────────────────────────────
                        let inner = outer_block.inner(popup_area);
                        let sections = Layout::default()
                            .direction(Direction::Vertical)
                            .margin(1)
                            .constraints([
                                Constraint::Length(1), // [0] title banner
                                Constraint::Length(1), // [1] gap
                                Constraint::Length(1), // [2] status row
                                Constraint::Length(1), // [3] due row
                                Constraint::Length(1), // [4] gap
                                Constraint::Length(3), // [5] description
                                Constraint::Length(1), // [6] gap
                                Constraint::Min(0),    // [7] tags
                            ])
                            .split(inner);

                        // ── title banner ─────────────────────────────────────────
                        let title_block = Block::default()
                            .borders(Borders::NONE)
                            .style(Style::default().bg(bg));

                        let title_para = Paragraph::new(Line::from(vec![Span::styled(
                            &task.title,
                            Style::default()
                                .fg(active)
                                .add_modifier(Modifier::BOLD),
                        )]))
                        .block(title_block)
                        .alignment(Alignment::Left)
                        .wrap(Wrap { trim: true });

                        frame.render_widget(title_para, sections[0]);

                        // ── status row ───────────────────────────────────────────
                        let (status_color, status_icon) = match task.status {
                            // replace with your actual Status variants, e.g.:
                            // Status::Done    => (Color::Indexed(114), "●"),
                            // Status::Pending => (Color::Indexed(179), "○"),
                            _ => (Color::Indexed(114), "●"),
                        };

                        let status_line = Line::from(vec![
                            Span::styled(
                                "Status ",
                                Style::default()
                                    .fg(active)
                                    .add_modifier(Modifier::BOLD),
                            ),
                            Span::styled(
                                format!("{} {:?}", status_icon, task.status),
                                Style::default()
                                    .fg(status_color)
                                    .add_modifier(Modifier::BOLD),
                            ),
                        ]);

                        frame.render_widget(Paragraph::new(status_line), sections[2]);

                        // ── due date row ─────────────────────────────────────────
                        let now = Local::now();
                        let (due_text, due_color) = if let Some(due) = task.due {
                            match (due - now).to_std() {
                                Ok(dur) => (
                                    format!("Due in {}", format_short_duration(dur)),
                                    Color::Indexed(226),
                                ),
                                Err(_) => (
                                    "Due     Overdue!".to_string(),
                                    Color::Indexed(203),
                                ),
                            }
                        } else {
                            ("No due date".to_string(), Color::Indexed(194))
                        };

                        frame.render_widget(
                            Paragraph::new(Line::from(vec![Span::styled(
                                due_text,
                                Style::default().fg(due_color),
                            )])),
                            sections[3],
                        );

                        // ── description ──────────────────────────────────────────
                        let desc_text = task.description.as_str();

                        let desc_lines = vec![
                            Line::from(vec![Span::styled(
                                "Description",
                                Style::default()
                                    .fg(active)
                                    .add_modifier(Modifier::BOLD),
                            )]),
                            Line::from(vec![Span::styled(
                                format!("  {}", desc_text),
                                Style::default().fg(Color::Indexed(194)),
                            )]),
                        ];

                        frame.render_widget(
                            Paragraph::new(desc_lines).wrap(Wrap { trim: true }),
                            sections[5],
                        );

                        // ── tags section ─────────────────────────────────────────
                        let tag_header = Line::from(vec![Span::styled(
                            "Tags",
                            Style::default()
                                .fg(active)
                                .add_modifier(Modifier::BOLD),
                        )]);

                        let tag_lines: Vec<Line> = if task.tags.is_empty() {
                            vec![
                                Line::from(tag_header),
                                Line::from(Span::styled(
                                    "no tags found",
                                    Style::default().fg(Color::Indexed(194)),
                                )),
                            ]
                        } else {
                            let mut lines = vec![Line::from(tag_header)];
                            for tag in &task.tags {
                                lines.push(Line::from(vec![
                                    Span::styled(
                                        "■ ",
                                        Style::default().fg(Color::Indexed(194)),
                                    ),
                                    Span::styled(
                                        tag.as_str(),
                                        Style::default().fg(Color::Indexed(194)),
                                    ),
                                ]));
                            }
                            lines
                        };

                        frame.render_widget(
                            Paragraph::new(tag_lines).wrap(Wrap { trim: true }),
                            sections[7],
                        );

                        // ── bottom hint bar ──────────────────────────────────────
                        let hint_area = Rect {
                            x: popup_area.x + 1,
                            y: popup_area.y + popup_area.height - 1,
                            width: popup_area.width.saturating_sub(2),
                            height: 1,
                        };

                        frame.render_widget(
                            Paragraph::new(Line::from(vec![Span::styled(
                                " [Esc] close",
                                Style::default().fg(active),
                            )]))
                            .alignment(Alignment::Right),
                            hint_area,
                        );
                    }
                }
            }
            _ => {}
        };
    }
}