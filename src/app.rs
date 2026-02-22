use ratatui::widgets::ListState;

use crate::api::WeatherResponse;

pub enum AppState {
    Idle,
    Loading,
    Error(String),
}

pub struct City {
    pub name: String,
    pub lat: f64,
    pub lon: f64,
}

pub struct App {
    pub cities: Vec<City>,
    pub list_state: ratatui::widgets::ListState,
    pub weather: Option<WeatherResponse>,
    pub state: AppState,
}

impl App {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            cities: vec![
                City { name: "Paris".into(), lat: 48.85, lon: 2.35 },
                City { name: "Tokio".into(), lat: 35.68, lon: 139.69 },
                City { name: "New York".into(), lat: 40.71, lon: -74.00 },
            ],
            list_state,
            weather: None,
            state: AppState::Idle,
        }
    }

    pub fn next(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => if i >= self.cities.len() - 1 { 0 } else { i + 1 },
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => if i >= self.cities.len() - 1 { 0 } else { i - 1 },
            None => 0,
        };
        self.list_state.select(Some(i));
    }
}
