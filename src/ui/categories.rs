use ratatui::{Frame, layout::Rect, style::{Color, Modifier, Style}, symbols::merge::MergeStrategy, widgets::{Block, BorderType, Borders, List, ListItem}};

use crate::{App, app::MainFocus};

impl App {
    pub fn render_categories(&mut self, frame: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self.categories
                    .iter()
                    .enumerate()
                    .map(|(i, category)| {
                        let count = if self.search_mode {
                            self.search_results
                                .iter()
                                .filter(|r| r.category_index == i)
                                .count()
                            } else {
                                category.taskslist.tasks.len()
                            };

                        let content = if self.search_mode {format!("{} ({})", category.title, count)} else {
                            format!("{}", category.title)
                        };

                        ListItem::new(content)
                    })
                    .collect();
        let color: Color = match self.mainfocus {
            MainFocus::Categories | MainFocus::SearchResults => self.config.active,
            _ => self.config.inactive,
        };

        let title = if self.search_mode {
            "Search"
        } else {
            "Categories"
        };

        let list = List::new(items)
            .block(
                Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
                .border_style(Style::default().fg(color))
                .merge_borders(MergeStrategy::Exact)
                .title(title).style(Style::default().bg(self.config.background))
            ).highlight_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Black).bg(color));
        frame.render_stateful_widget(list, area, &mut self.categoryliststate);
    }
}
