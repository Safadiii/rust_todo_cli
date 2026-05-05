use ratatui::{Frame, layout::Rect, style::{Color, Style, Stylize}, text::Text, widgets::{Block, BorderType, Borders, Paragraph}};

use crate::{App, app::{CmdMode, MainFocus}};

impl App {
    pub fn render_command_center(&mut self, frame: &mut Frame, area: Rect) {
        let color = match self.mainfocus {
            MainFocus::None => {
                self.config.ui.active
            }
            _ => self.config.ui.inactive
        };

        let title: String = match self.commandmode {
            CmdMode::AddingCategory => String::from("Category Name"),
            CmdMode::AddingDescription => String::from("Description"),
            CmdMode::Search => String::from("Search"),
            _ => String::from("Command")
        };

        let cmd_input = Text::from(self.cmd.as_str());
        let block = Block::default().borders(Borders::ALL).border_type(BorderType::Thick).border_style(Style::default().fg(color)).title(title).bg(self.config.ui.background);
        let inner = block.inner(area);
        let cmd = Paragraph::new(cmd_input).block(block);
        frame.render_widget(cmd, area);
        match self.mainfocus {
            MainFocus::None => {frame.set_cursor_position((inner.x + self.cmd.chars().count() as u16, inner.y));} 
            _ => {}
        }
    }
}