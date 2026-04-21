use ratatui::{Frame, layout::Rect, style::{Color, Style}, text::Text, widgets::{Block, Borders, Paragraph}};

use crate::{App, app::{CmdMode, MainFocus}};

impl App {
    pub fn render_command_center(&mut self, frame: &mut Frame, area: Rect) {
        let color = match self.mainfocus {
            MainFocus::None => {
                Color::Indexed(73)
            }
            _ => Color::Indexed(250)
        };

        let title: String = match self.commandmode {
            CmdMode::AddingCategory => String::from("Category Name"),
            CmdMode::AddingDescription => String::from("Description"),
            _ => String::new()
        };

        let cmd_input = Text::from(self.cmd.as_str());
        let block = Block::default().borders(Borders::ALL).border_style(Style::default().fg(color)).title(title);
        let inner = block.inner(area);
        let cmd = Paragraph::new(cmd_input).block(block);
        frame.render_widget(cmd, area);
        match self.mainfocus {
            MainFocus::None => {frame.set_cursor_position((inner.x + self.cmd.chars().count() as u16, inner.y));} 
            _ => {}
        }
    }
}