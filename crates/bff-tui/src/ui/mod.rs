mod editor;
mod footer;
mod title;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
};

use super::app::App;

pub fn ui(frame: &mut Frame, app: &App) {
    // Layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(frame.area());

    self::title::render(frame, chunks[0]);
    if let Err(e) = self::editor::render(frame, chunks[1], app) {
        eprintln!("{}", e);
    }
    self::footer::render(frame, chunks[2], app);
}
