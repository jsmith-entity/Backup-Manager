use crossterm::event::KeyCode;

use ratatui::{
    layout::Flex,
    prelude::{Buffer, Constraint::Percentage, Layout, Rect},
    text::Line,
    widgets::{Block, Widget},
};

use crate::{
    KeyConfig,
    components::{Component, NewConfigFormComponent},
    events::EventState,
};

#[derive(Clone)]
pub struct BackupConfigComponent {
    new_config_popup: NewConfigFormComponent,
    key_config: KeyConfig,
    // table for the list of backup configs
}

impl BackupConfigComponent {
    pub fn new(key_config: KeyConfig) -> Self {
        return Self {
            new_config_popup: NewConfigFormComponent::new(key_config),
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

    pub fn toggle_popup(&mut self, visible: bool) {
        self.new_config_popup.visible = visible;
    }
}

impl Component for BackupConfigComponent {
    fn event(&mut self, key: KeyCode) -> anyhow::Result<EventState> {
        if self.new_config_popup.visible && self.new_config_popup.event(key)?.is_consumed() {
            return Ok(EventState::Consumed);
        }

        // navigate through the list

        return Ok(EventState::NotConsumed);
    }

    fn render(&self, area: &[Rect], buf: &mut Buffer) {
        let block = Block::bordered().title("Backup Configs");
        let inner_area = block.inner(area[1]);
        block.render(area[1], buf);

        Line::from("this is the config list area").render(inner_area, buf);

        if self.new_config_popup.visible {
            let popup_area = BackupConfigComponent::center_area(area[0]);
            self.new_config_popup.render(&[popup_area], buf);
        }
    }
}
