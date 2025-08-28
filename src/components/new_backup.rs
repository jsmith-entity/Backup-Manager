use crossterm::event::KeyCode;

use ratatui::{
    layout::Flex,
    prelude::{
        Alignment, Buffer,
        Constraint::{Length, Min, Percentage},
        Layout, Rect, Stylize,
    },
    style::Color,
    text::Line,
    widgets::{Block, Clear, Paragraph, Widget},
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

    pub fn center_area(area: Rect) -> Rect {
        let vertical = Layout::vertical([Percentage(50)]).flex(Flex::Center);
        let horizontal = Layout::horizontal([Percentage(50)]).flex(Flex::Center);
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

        if key == self.key_config.confirm {
            // push new backup
            // maybe rename this to just backups or something
            self.visible = false;
            return Ok(EventState::Consumed);
        }

        return Ok(EventState::NotConsumed);
    }
}

impl Widget for NewBackupComponent {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Clear.render(area, buf);

        let block = Block::bordered().title_top(Line::from("Create a New Backup").centered());
        let inner_area = block.inner(area);
        block.render(area, buf);

        let vertical = Layout::vertical([Min(0), Length(1)]);
        let [main_area, control_area] = vertical.areas(inner_area);

        self.render_controls(control_area, buf);
    }
}

impl NewBackupComponent {
    fn render_controls(&self, area: Rect, buf: &mut Buffer) {
        let centered_area = NewBackupComponent::center_area(area);
        let horizontal = Layout::horizontal([Min(0), Length(1), Min(0)]);
        let [confirm_area, _, cancel_area] = horizontal.areas(centered_area);

        Paragraph::new("(↵) Confirm")
            .alignment(Alignment::Center)
            .block(Block::default())
            .fg(Color::Black)
            .bg(Color::Gray)
            .render(confirm_area, buf);

        Paragraph::new("(q) Cancel")
            .alignment(Alignment::Center)
            .block(Block::default())
            .fg(Color::Black)
            .bg(Color::Gray)
            .render(cancel_area, buf);
    }
}
