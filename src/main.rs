mod app;
mod config;
mod events;

use std::time::Duration;

use crate::{app::App, config::KeyConfig, events::*};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let key_config = KeyConfig::default();
    let mut app = App::new(key_config);

    let tick_rate = Duration::from_millis(200);
    let events = Events::new(tick_rate);
    let mut terminal = ratatui::init();

    loop {
        terminal
            .draw(|frame| frame.render_widget(&app, frame.area()))
            .expect("failed to draw frame");

        match events.next()? {
            InputEvent::Input(key) => match app.event(key.code).await {
                Ok(state) => {
                    if !state.is_consumed() && key.code == app.key_config.quit {
                        break;
                    }
                }
                Err(e) => {
                    println!("{}", e);
                }
            },
            InputEvent::Tick => (),
        }
    }

    ratatui::restore();

    Ok(())
}
