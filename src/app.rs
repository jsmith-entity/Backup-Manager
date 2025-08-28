use crossterm::event::KeyCode;
use ratatui::{
    prelude::{Buffer, Rect},
    widgets::Widget,
};

use crate::{config::KeyConfig, events::EventState};

pub struct App {
    pub key_config: KeyConfig,
}

impl App {
    pub fn new(key_config: KeyConfig) -> Self {
        return Self { key_config };
    }

    pub async fn event(&mut self, key: KeyCode) -> anyhow::Result<EventState> {
        return Ok(EventState::NotConsumed);
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // erm
    }
}
