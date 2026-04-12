use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Paragraph},
};

use crate::{
    app::{App, WrapLine, bytes_to_lines},
    error::Result,
};

pub fn render(frame: &mut Frame, rect: Rect, app: &App) -> Result<()> {
    let output_block = Block::bordered()
        .border_style(Style::default())
        .title_alignment(Alignment::Center)
        .title("Output");
    let output_bytes = app.output()?;
    let output_data = bytes_to_lines(
        output_bytes.as_slice(),
        WrapLine::Width(rect.width as usize),
    );
    let text = Text::from(output_data).style(Style::default().fg(Color::White));
    let output = Paragraph::new(text).block(output_block);
    frame.render_widget(output, rect);

    Ok(())
}
