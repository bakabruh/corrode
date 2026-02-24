use crate::app::{App, AppState};
use ratatui::{
    layout::{Constraint, Direction, HorizontalAlignment, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(f.area());

    let show_cities = app.input_text.len() >= 1;
    let city_count = if show_cities {
        app.filtered_cities.len()
    } else {
        0
    };
    let city_block_height = if city_count > 0 {
        city_count as u16 + 2
    } else {
        0
    };

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Length(city_block_height)])
        .split(chunks[0]);

    let input_block = Block::default()
        .title(" Filter City ")
        .borders(Borders::ALL);

    let input_text = format!("> {}", app.input_text);

    let input = Paragraph::new(input_text)
        .block(input_block)
        .style(Style::default().fg(Color::Blue));

    f.render_widget(input, left_chunks[0]);

    if show_cities && city_count > 0 {
        let mut city_lines: Vec<Line> = Vec::new();
        for (i, city) in app.filtered_cities.iter().enumerate() {
            if i == app.selection_index {
                city_lines.push(Line::from(vec![
                    Span::raw("> "),
                    Span::styled(city.name.clone(), Style::default().bold()),
                ]));
            } else {
                city_lines.push(Line::from(format!("  {}", city.name)));
            }
        }

        let city_block = Block::default().title(" Cities ").borders(Borders::ALL);

        let city_text = Paragraph::new(city_lines)
            .block(city_block)
            .style(Style::default().fg(Color::Blue));

        f.render_widget(city_text, left_chunks[1]);
    }

    let detail_block = Block::default()
        .title_alignment(HorizontalAlignment::Center)
        .title(" Current Weather ")
        .borders(Borders::ALL);

    let content = match &app.state {
        AppState::Loading => Paragraph::new("Loading..."),
        AppState::Error(e) => Paragraph::new(e.clone()),
        AppState::Idle => {
            if let Some(w) = &app.weather {
                Paragraph::new(format!(
                    "Temperature: {}°C\nWind: {} km/h",
                    w.current_weather.temperature, w.current_weather.windspeed
                ))
            } else {
                Paragraph::new("Select a city and press Enter to load weather")
            }
        }
    };

    f.render_widget(content.block(detail_block), chunks[1]);
}
