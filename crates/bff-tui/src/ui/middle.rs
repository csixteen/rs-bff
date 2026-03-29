use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Position, Rect},
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, Paragraph},
};

use crate::{
    Result,
    app::{App, CurrentScreen},
};

pub fn render<'a>(frame: &'a mut Frame, rect: Rect, app: &'a App) -> Result<()> {
    let middle_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(rect);

    // Left-hand side, the editor
    let input_block = Block::bordered()
        .border_style(if matches!(app.current_screen(), CurrentScreen::Editing) {
            Style::new().yellow()
        } else {
            Style::default()
        })
        .title("Input");
    let input_data = app.input_to_lines(middle_layout[0].width as usize - 2)?;
    let text = Text::from(input_data).style(Style::default().fg(Color::White));
    let new_lines = text.height().saturating_sub(1);
    let input = Paragraph::new(text).block(input_block);
    frame.render_widget(input, middle_layout[0]);

    // Render cursor
    if matches!(app.current_screen(), CurrentScreen::Editing)
        && matches!(app.editing_mode(), crate::app::EditingMode::Insert)
    {
        frame.set_cursor_position(Position::new(
            middle_layout[0].x
                + (app.cursor_index() as u16 % (middle_layout[0].width - 1))
                + 1
                + new_lines as u16,
            middle_layout[0].y + (app.cursor_index() as u16 / (middle_layout[0].width - 1)) + 1,
        ));
    }

    // Right-hand side, the debug info

    Ok(())
}
