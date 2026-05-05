use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::Frame;
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Clear, Padding, Paragraph, Wrap};

use crate::app::{AddTaskField, App};


impl App {
    pub fn render_add_task_popup(&mut self, frame: &mut Frame, area: Rect) {
        self.clamp_cursor();
        let color_main = self.config.ui.active;
        let bg = self.config.ui.background;
        let mode_span = if self.editing_task_id.is_some() {
            Span::styled(" EDIT ", Style::default().bg(color_main).fg(Color::Black).add_modifier(Modifier::BOLD))
        } else {
            Span::styled(" ADD ", Style::default().bg(color_main).fg(Color::Black).add_modifier(Modifier::BOLD))
        };

        let title = Line::from(vec![
            Span::raw("Add Task "),
            mode_span,
        ]);

        let add_task_block = Block::bordered()
            .title(title)
            .fg(color_main)
            .bg(bg);
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
                _ => Style::default().fg(self.config.ui.inactive),
            }
        );
        let tags_block = Block::default().title("Tags").borders(Borders::ALL).style(
            match self.addtaskfield {
                AddTaskField::Tags => Style::default().fg(color_main),
                _ => Style::default().fg(self.config.ui.inactive),
            }
        );
        let due_block = Block::default().title("Due").borders(Borders::ALL).style(
            match self.addtaskfield {
                AddTaskField::Due => Style::default().fg(color_main),
                _ => Style::default().fg(self.config.ui.inactive),
            }
        );  
        let recurring_block = Block::default().title("Recurring").borders(Borders::ALL).style(
            match self.addtaskfield {
                AddTaskField::Recurring => Style::default().fg(color_main),
                _ => Style::default().fg(self.config.ui.inactive),
            }
        );     

        let hint_area = Rect {
            x: popup_area.x + 1,
            y: popup_area.y + popup_area.height,
            width: popup_area.width.saturating_sub(2),
            height: 1,
        };

        let hint_area_2 = Rect {
            x: popup_area.x + 1,
            y: popup_area.y + popup_area.height - 1,
            width: popup_area.width.saturating_sub(2),
            height: 1,
        };

        let mode: &str = if self.inputtingmode {
            " EDITING " 
        } else {
            " VIEWING "
        };

        let cmd: &str = if self.inputtingmode {
            "[ESC] VIEWING MODE "
        } else {
            "[E] EDITING MODE "
        };



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
        frame.render_widget(
            Paragraph::new(Line::from(vec![Span::styled(
                mode,
                Style::default().fg(color_main),
            )]))
            .alignment(Alignment::Right),
            hint_area,
        );
        frame.render_widget(
            Paragraph::new(Line::from(vec![Span::styled(
                cmd,
                Style::default().fg(color_main),
            )]))
            .alignment(Alignment::Left),
            hint_area_2,
        );

        self.render_cursor(frame, (task_title_area, task_tags_area, task_due_area, reccuring_area));
    }
}