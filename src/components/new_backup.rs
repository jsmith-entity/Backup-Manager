use crossterm::event::KeyCode;

use ratatui::{
    layout::Flex,
    prelude::{Buffer, Constraint, Layout, Rect},
    widgets::{Block, Clear, Widget},
};

use crate::{KeyConfig, components::Component, events::EventState};

#[derive(Default, Clone, Copy)]
pub struct NewBackupComponent {
    pub visible: bool,
    key_config: KeyConfig,
}

impl NewBackupComponent {
    pub fn new(key_config: KeyConfig) -> Self {
        return Self {
            visible: false,
            key_config,
        };
    }
    pub fn area(area: Rect) -> Rect {
        let vertical = Layout::vertical([Constraint::Percentage(50)]).flex(Flex::Center);
        let horizontal = Layout::horizontal([Constraint::Percentage(50)]).flex(Flex::Center);
        let [area] = vertical.areas(area);
        let [area] = horizontal.areas(area);
        return area;
    }
}

impl Component for NewBackupComponent {
    fn event(&mut self, key: KeyCode) -> anyhow::Result<EventState> {
        if !self.visible {
            return Ok(EventState::NotConsumed);
        }

        if key == self.key_config.quit {
            self.visible = false;
            return Ok(EventState::Consumed);
        }

        return Ok(EventState::NotConsumed);
    }
}

impl Widget for NewBackupComponent {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Clear.render(area, buf);

        Block::bordered().title("erm popup").render(area, buf);
    }
}
