use ratatui::{Frame, layout::Rect, style::{Color, Modifier, Style}, symbols::merge::MergeStrategy, widgets::{Block, BorderType, Borders, List, ListItem}};

use crate::{App, app::MainFocus};

impl App {
    pub fn render_categories(&mut self, frame: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self.categories
                    .iter()
                    .map(|category| {

                        let content = format!("{}", category.title);

                        ListItem::new(content)
                    })
                    .collect();
        let color: Color = match self.mainfocus {
            MainFocus::Categories => self.config.active,
            _ => self.config.inactive,
        };
        let list = List::new(items)
            .block(
                Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
                .border_style(Style::default().fg(color))
                .merge_borders(MergeStrategy::Exact)
                .title("Categories").style(Style::default().bg(self.config.background))
            ).highlight_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Black).bg(color));
        frame.render_stateful_widget(list, area, &mut self.categoryliststate);
    }
}
