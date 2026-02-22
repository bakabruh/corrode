mod api;
mod app;
mod ui;
mod handler;

use crate::app::AppState;
use std::{error::Error, io, time::Duration};
use crossterm::{event::{self, Event, KeyCode}, terminal::*};
use ratatui::{backend::CrosstermBackend, Terminal};
use tokio::sync::mpsc;
use crate::app::App;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    let mut app = App::new();
    let (tx, mut rx) = mpsc::channel(10);

    loop {
        terminal.draw(|f| ui::render(f, &mut app))?;

        if let Ok(data) = rx.try_recv() {
            app.weather = Some(data);
            app.state = AppState::Idle;
        }

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }

                handler::handle_key_events(key, &mut app, tx.clone());
            }
        }
    }

    disable_raw_mode();
    Ok(())
}
