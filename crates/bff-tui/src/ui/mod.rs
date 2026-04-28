mod footer;
mod middle;
mod output;
mod title;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
};

use super::app::App;

pub fn ui(frame: &mut Frame, app: &mut App) {
    // Layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(10),
            Constraint::Length(3),
        ])
        .split(frame.area());

    self::title::render(frame, chunks[0]);
    if let Err(e) = self::middle::render(frame, chunks[1], app) {
        eprintln!("{}", e);
    }
    if let Err(e) = self::output::render(frame, chunks[2], app) {
        eprintln!("{}", e);
    }
    self::footer::render(frame, chunks[3], app);
}
