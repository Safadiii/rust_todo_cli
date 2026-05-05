use ratatui::{Frame, layout::{Alignment, Constraint, Rect}, style::{Color, Modifier, Style, Stylize}, text::{Line, Span}, widgets::{Block, Borders, Paragraph, Wrap}};

use crate::App;


impl App {
    pub fn render_help_screen(&mut self, frame: &mut Frame, area: Rect) {
        let popup_area = area.centered(
            Constraint::Percentage(80),
            Constraint::Percentage(80),
        );

        let color = self.config.ui.active;

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
}