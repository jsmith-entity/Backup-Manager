use crossterm::event::KeyCode;
use ratatui::{
    prelude::{Buffer, Rect},
    text::Line,
    widgets::{Block, Widget},
};

use crate::{EventState, components::Component};

#[derive(Clone)]
pub struct TextField {
    pub title: String,
    pub input: String,
}

impl TextField {
    pub fn new(title: &str) -> Self {
        return Self {
            title: title.to_string(),
            input: String::new(),
        };
    }
}

impl Component for TextField {
    fn event(&mut self, key: KeyCode) -> anyhow::Result<EventState> {
        return Ok(EventState::NotConsumed);
    }

    fn render(&self, area: &[Rect], buf: &mut Buffer) {
        let block = Block::bordered().title(Line::from(self.title.as_str()));
        let inner_area = block.inner(area[0]);
        block.render(area[0], buf);

        Line::from(self.input.as_str()).render(inner_area, buf);
    }
}
