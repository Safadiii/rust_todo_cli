use crossterm::event::{KeyCode, KeyEvent};
use color_eyre::Result;

use crate::app::{App, Focus};

impl App {
    pub fn handle_helpkeys(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Esc => {
                self.focus = Focus::None;
            }
            _ => {}
        }
        Ok(())
    }
}