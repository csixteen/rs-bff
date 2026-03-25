use std::error::Error;

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;

pub fn render<'a>(
    frame: &'a mut Frame,
    rect: Rect,
    app: &'a App,
) -> Result<(), Box<dyn Error + 'a>> {
    let input_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());
    let input = Paragraph::new(Text::styled(
        app.input()?,
        Style::default().fg(Color::White),
    ))
    .block(input_block);
    frame.render_widget(input, rect);

    Ok(())
}
