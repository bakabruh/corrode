use crossterm::event::{KeyEvent, KeyCode};
use tokio::sync::mpsc;

use crate::{api, app::{App, AppState}};

pub fn handle_key_events(key_event: KeyEvent, app: &mut App, tx: mpsc::Sender<api::WeatherResponse>) {
    match key_event.code {
        KeyCode::Up => app.previous(),
        KeyCode::Down => app.next(),
        KeyCode::Enter => {
            if let Some(index) = app.list_state.selected() {
                let city = &app.cities[index];
                app.state = AppState::Loading;

                let tx_clone = tx.clone();
                let (lat, lon) = (city.lat, city.lon);

                tokio::spawn(async move {
                    if let Ok(data) = api::fetch_weather(lat, lon).await {
                        let _ = tx_clone.send(data).await;
                    }
                });
            }
        },
        _ => {},
    }
}
