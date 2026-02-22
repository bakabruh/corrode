use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use crate::app::{App, AppState};

pub fn render(f: &mut Frame, app: &mut App) {
    // Découpage de l'écran : Gauche (Liste) | Droite (Détails)
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(f.area());

    // 1. Liste des villes
    let items: Vec<ListItem> = app.cities
        .iter()
        .map(|c| ListItem::new(c.name.as_str()))
        .collect();

    let list = List::new(items)
        .block(Block::default().title(" Villes ").borders(Borders::ALL))
        .highlight_symbol(">> ")
        .highlight_style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow));

    f.render_stateful_widget(list, chunks[0], &mut app.list_state);

    // 2. Affichage Météo
    let detail_block = Block::default().title(" Météo Actuelle ").borders(Borders::ALL);
    
    let content = match &app.state {
        AppState::Loading => Paragraph::new("Loading..."),
        AppState::Error(e) => Paragraph::new(format!("Error: {}", e)),
        AppState::Idle => {
            if let Some(w) = &app.weather {
                Paragraph::new(format!(
                    "Temperature: {}°C\nWind: {} km/h",
                    w.current_weather.temperature, w.current_weather.windspeed
                ))
            } else {
                Paragraph::new("Press Enter to load")
            }
        }
    };

    f.render_widget(content.block(detail_block), chunks[1]);
}
