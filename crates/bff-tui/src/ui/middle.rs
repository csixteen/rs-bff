use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Paragraph},
};

use crate::{Result, app::App};

pub fn render<'a>(frame: &'a mut Frame, rect: Rect, app: &'a App) -> Result<()> {
    let middle_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(rect);

    let input_block = Block::bordered()
        .border_style(Style::default())
        .title_alignment(Alignment::Center)
        .title("Use h j k l or ◄ ▲ ▼ ► to scroll");
    let input_data = app.program_to_lines(middle_layout[0].width as usize - 2);
    let text = Text::from(input_data).style(Style::default().fg(Color::White));
    let input = Paragraph::new(text).block(input_block);
    frame.render_widget(input, middle_layout[0]);

    let debug_block = Block::bordered()
        .border_style(Style::default())
        .title_alignment(Alignment::Center)
        .title("Debug info");
    let debug_text = Text::from(app.debug_info()).style(Style::default().fg(Color::Blue));
    let debug = Paragraph::new(debug_text).block(debug_block);
    frame.render_widget(debug, middle_layout[1]);

    Ok(())
}
