use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
};

pub fn render(frame: &mut Frame, rect: Rect) {
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());
    let title = Paragraph::new(Text::styled(
        "Brainfuck interpreter",
        Style::default().fg(Color::Green),
    ))
    .block(title_block)
    .centered();
    frame.render_widget(title, rect);
}
