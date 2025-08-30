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
    components::{BackupConfigComponent, Component, Tab},
    config::KeyConfig,
    events::EventState,
};

pub struct App {
    tab: Tab,
    backup_config: BackupConfigComponent,
    pub key_config: KeyConfig,
}

impl App {
    pub fn new(key_config: KeyConfig) -> Self {
        return Self {
            tab: Tab::MAIN,
            backup_config: BackupConfigComponent::new(key_config),
            key_config,
        };
    }

    pub async fn event(&mut self, key: KeyCode) -> anyhow::Result<EventState> {
        if self.backup_config.event(key)?.is_consumed() {
            return Ok(EventState::Consumed);
        }

        if key == self.key_config.new {
            self.backup_config.toggle_popup(true);
            return Ok(EventState::Consumed);
        }

        return Ok(EventState::NotConsumed);
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([Length(1), Min(0)]);
        let [header_area, body_area] = vertical.areas(area);

        let header_horizontal = Layout::horizontal([Min(0), Length(20)]);
        let [tabs_area, title_area] = header_horizontal.areas(header_area);

        self.tab.render(tabs_area, buf);
        Line::from("Backup Manager").bold().render(title_area, buf);

        let block = Block::bordered()
            .border_set(symbols::border::PROPORTIONAL_TALL)
            .padding(Padding::horizontal(1))
            .border_style(Color::Gray);
        let inner_area = block.inner(body_area);
        block.render(body_area, buf);

        let body_horizontal = Layout::horizontal([Min(0), Min(0), Min(0)]);
        let [config_area, list_area, options_area] = body_horizontal.areas(inner_area);

        self.backup_config.render(&[inner_area, config_area], buf);
        Line::from("this is the backup list area").render(list_area, buf);
        Line::from("this is the options area").render(options_area, buf);
    }
}
