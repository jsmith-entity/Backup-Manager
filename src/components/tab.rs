use ratatui::{
    prelude::{Buffer, Rect, Stylize},
    style::Color,
    text::Line,
    widgets::{Tabs, Widget},
};

use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Display, EnumIter, Clone, Copy)]
pub enum Tab {
    #[strum(to_string = "(1) Backups")]
    MAIN,
    #[strum(to_string = "(2) Log")]
    LOG,
    #[strum(to_string = "(3) Debug")]
    DEBUG,
}

impl Tab {
    fn title(self) -> Line<'static> {
        return format!("  {self}  ").fg(Color::Gray).into();
    }
}

impl Widget for Tab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let titles = Tab::iter().map(Tab::title);

        let selected_idx = self as usize;
        let hightlight_style = (Color::Black, Color::Gray);

        Tabs::new(titles)
            .highlight_style(hightlight_style)
            .select(selected_idx)
            .padding("", "")
            .divider(" ")
            .render(area, buf);
    }
}
