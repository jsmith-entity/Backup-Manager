pub mod backup_config;
pub mod tab;

mod forms;
mod new_config_form;

use new_config_form::NewConfigFormComponent;

pub use backup_config::BackupConfigComponent;
pub use tab::Tab;

use crate::events::EventState;
use crossterm::event::KeyCode;
use ratatui::prelude::{Buffer, Rect};

pub trait Component {
    fn event(&mut self, key: KeyCode) -> anyhow::Result<EventState>;

    fn render(&self, area: &[Rect], buf: &mut Buffer);
}
