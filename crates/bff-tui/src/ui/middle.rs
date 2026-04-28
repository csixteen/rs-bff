use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Paragraph, Scrollbar, ScrollbarOrientation},
};

use crate::{
    Result,
    app::{App, WrapLine},
};

pub fn render<'a>(frame: &'a mut Frame, rect: Rect, app: &'a mut App) -> Result<()> {
    let middle_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(rect);

    let input_block = Block::bordered()
        .border_style(Style::default())
        .title_alignment(Alignment::Center)
        .title("Use h j k l or ◄ ▲ ▼ ► to scroll");
    let input_data = app.program_to_lines(WrapLine::Never);
    let text = Text::from(input_data).style(Style::default().fg(Color::White));
    let input = Paragraph::new(text)
        .block(input_block)
        .scroll((app.vertical_scroll() as u16, 0));
    frame.render_widget(input, middle_layout[0]);
    frame.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓")),
        middle_layout[0],
        app.vertical_scroll_state_mut(),
    );

    let debug_block = Block::bordered()
        .border_style(Style::default())
        .title_alignment(Alignment::Center)
        .title("Debug info");
    let debug_text = Text::from(app.debug_info()).style(Style::default().fg(Color::Blue));
    let debug = Paragraph::new(debug_text).block(debug_block);
    frame.render_widget(debug, middle_layout[1]);

    Ok(())
}
