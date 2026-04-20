use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::Frame;
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Clear, Padding, Paragraph, Wrap};

use crate::app::{AddTaskField, App};


impl App {
    pub fn render_add_task_popup(&mut self, frame: &mut Frame, area: Rect) {
        self.clamp_cursor();
        let color_main = Color::Indexed(73);
        let mode_span = if self.editing_task_id.is_some() {
            Span::styled(" EDIT ", Style::default().bg(Color::Yellow).fg(Color::Black))
        } else {
            Span::styled(" ADD ", Style::default().bg(Color::Green).fg(Color::Black))
        };

        let title = Line::from(vec![
            Span::raw("Add Task "),
            mode_span,
        ]);

        let add_task_block = Block::bordered()
            .title(title)
            .fg(color_main);
        let centered_area = area.centered(Constraint::Percentage(50), Constraint::Max(15));

        let popup_height = 13;


        let popup_area = centered_area.centered(Constraint::Fill(1), Constraint::Length(popup_height as u16));

        let block = Block::default().padding(Padding::horizontal(1)).inner(popup_area);

        
        let details_layout = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3)
        ]);


        let [task_title_area, task_tags_area, task_due_area, reccuring_area] = block.layout(&details_layout);

        let title_block = Block::default().title("Title").borders(Borders::ALL).style(
            match self.addtaskfield {
                AddTaskField::Title => Style::default().fg(color_main),
                _ => Style::default().fg(Color::Indexed(250)),
            }
        );
        let tags_block = Block::default().title("Tags").borders(Borders::ALL).style(
            match self.addtaskfield {
                AddTaskField::Tags => Style::default().fg(color_main),
                _ => Style::default().fg(Color::Indexed(250)),
            }
        );
        let due_block = Block::default().title("Due").borders(Borders::ALL).style(
            match self.addtaskfield {
                AddTaskField::Due => Style::default().fg(color_main),
                _ => Style::default().fg(Color::Indexed(250)),
            }
        );  
        let recurring_block = Block::default().title("Recurring").borders(Borders::ALL).style(
            match self.addtaskfield {
                AddTaskField::Due => Style::default().fg(color_main),
                _ => Style::default().fg(Color::Indexed(250)),
            }
        );     


        let text = Text::from(self.title_input.as_str());
        let tags_text = Text::from(self.tags_input.as_str());
        let due_text = Text::from(self.due_input.as_str());
        let recurring_text = Text::from(self.recurrence_input.as_str());


        let title = Paragraph::new(text).block(title_block).wrap(Wrap { trim: false});
        let tags = Paragraph::new(tags_text).block(tags_block);
        let due = Paragraph::new(due_text).block(due_block);
        let recurrence = Paragraph::new(recurring_text).block(recurring_block);


        frame.render_widget(Clear, centered_area);

        frame.render_widget(add_task_block, centered_area);

        frame.render_widget(title, task_title_area);
        frame.render_widget(due, task_due_area);
        frame.render_widget(tags, task_tags_area);
        frame.render_widget(recurrence, reccuring_area);

        self.render_cursor(frame, (task_title_area, task_tags_area, task_due_area, reccuring_area));
    }
}