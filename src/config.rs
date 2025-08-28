use crossterm::event::KeyCode;

#[derive(Clone, Copy)]
pub struct KeyConfig {
    pub quit: KeyCode,
    pub new: KeyCode,
    pub confirm: KeyCode,
}

impl Default for KeyConfig {
    fn default() -> Self {
        Self {
            quit: KeyCode::Char('q'),
            new: KeyCode::Char('n'),
            confirm: KeyCode::Enter,
        }
    }
}
