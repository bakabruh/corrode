mod api;
mod app;
mod ui;

use std::io;

use crossterm::event::KeyEvent;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

use crate::api::WeatherResponse;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
}
