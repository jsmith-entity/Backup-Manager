pub mod new_backup;
pub mod tab;

pub use new_backup::NewBackupComponent;
pub use tab::Tab;

use crate::events::EventState;
use crossterm::event::KeyCode;

pub trait Component {
    fn event(&mut self, key: KeyCode) -> anyhow::Result<EventState>;
}
