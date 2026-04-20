use ratatui::{Frame, layout::{Alignment, Rect}, style::{Color, Modifier, Style}, text::{Line, Span}, widgets::{Block, Borders, Paragraph}};

use crate::{App, app::Focus};

impl App {
    pub fn _render_footer(&self, frame: &mut Frame, area: Rect) {
        let controls: Vec<Line<>> = match self.focus {
            Focus::AddTaskPopup => {
                if self.inputtingmode {
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
}