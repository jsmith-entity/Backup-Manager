use crossterm::event::KeyCode;
use ratatui::{
    prelude::{
        Buffer,
        Constraint::{Length, Min},
        Layout, Rect, Stylize,
    },
    style::Color,
    symbols,
    text::Line,
    widgets::{Block, Padding, Widget},
};

use crate::{
    components::{Component, NewBackupComponent, Tab},
    config::KeyConfig,
    events::EventState,
};

pub struct App {
    tab: Tab,
    new_backup: NewBackupComponent,
    pub key_config: KeyConfig,
}

impl App {
    pub fn new(key_config: KeyConfig) -> Self {
        return Self {
            tab: Tab::MAIN,
            new_backup: NewBackupComponent::new(key_config),
            key_config,
        };
    }

    pub async fn event(&mut self, key: KeyCode) -> anyhow::Result<EventState> {
        if self.new_backup.event(key)?.is_consumed() {
            return Ok(EventState::Consumed);
        }

        if key == self.key_config.new {
            self.new_backup.visible = true;
            return Ok(EventState::Consumed);
        }

        return Ok(EventState::NotConsumed);
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([Length(1), Min(0)]);
        let [header_area, body_area] = vertical.areas(area);

        let horizontal = Layout::horizontal([Min(0), Length(20)]);
        let [tabs_area, title_area] = horizontal.areas(header_area);

        self.tab.render(tabs_area, buf);
        Line::from("Backup Manager").bold().render(title_area, buf);

        let block = Block::bordered()
            .border_set(symbols::border::PROPORTIONAL_TALL)
            .padding(Padding::horizontal(1))
            .border_style(Color::Gray);
        let inner_area = block.inner(body_area);
        block.render(body_area, buf);

        if self.new_backup.visible {
            let popup_area = NewBackupComponent::area(inner_area);
            self.new_backup.render(popup_area, buf);
        }

        // render more
    }
}
