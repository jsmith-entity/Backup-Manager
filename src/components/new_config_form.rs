use crossterm::event::KeyCode;
use ratatui::{
    prelude::{
        Alignment, Buffer,
        Constraint::Percentage,
        Constraint::{Length, Min},
        Layout, Rect, Stylize,
    },
    style::Color,
    text::Line,
    widgets::{Block, Clear, Paragraph, Widget},
};

use crate::{
    EventState,
    components::{BackupConfigComponent, Component, forms::TextField, utils},
    config::KeyConfig,
};

#[derive(Clone)]
pub struct NewConfigFormComponent {
    pub visible: bool,
    config_name: TextField,
    key_config: KeyConfig,
}

impl NewConfigFormComponent {
    pub fn new(key_config: KeyConfig) -> Self {
        return Self {
            visible: false,
            config_name: TextField::new("title"),
            key_config,
        };
    }
}

impl Component for NewConfigFormComponent {
    fn event(&mut self, key: KeyCode) -> anyhow::Result<EventState> {
        if key == self.key_config.quit {
            self.visible = false;
            return Ok(EventState::Consumed);
        }

        if key == self.key_config.confirm {
            // push new backup
            // maybe rename this to just backups or something
            //
            self.visible = false;
            return Ok(EventState::Consumed);
        }

        return Ok(EventState::NotConsumed);
    }

    fn render(&self, area: &[Rect], buf: &mut Buffer) {
        let area = area[0];
        Clear.render(area, buf);

        let block = Block::bordered().title_top(Line::from("Create a New Backup").centered());
        let inner_area = block.inner(area);
        block.render(area, buf);

        let vertical = Layout::vertical([Min(0), Length(1)]);
        let [main_area, control_area] = vertical.areas(inner_area);

        self.render_popup_form(main_area, buf);
        self.render_popup_controls(control_area, buf);
    }
}

impl NewConfigFormComponent {
    fn render_popup_form(&self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([Length(3)]);
        let [title_area] = vertical.areas(area);
        self.config_name.render(&[title_area], buf);
    }

    fn render_popup_controls(&self, area: Rect, buf: &mut Buffer) {
        let centered_area = utils::center(area, Percentage(50), Percentage(50));
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
