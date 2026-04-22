use ratatui::{Frame, layout::{Constraint, Direction, Layout, Spacing}, style::{Color, Style}, widgets::Block};

use crate::{App, app::Focus};

impl App {
    pub fn draw(&mut self, frame: &mut Frame) {
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
        match self.focus {
            Focus::Search => {self.render_results(frame, top_chunks[0]);}
            _ => {self.render_categories(frame, top_chunks[0]);
}
        }
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