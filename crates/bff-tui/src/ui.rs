use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
};

use super::app::{App, CurrentScreen, EditorMode, RunningMode};

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

    // Title
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());
    let title = Paragraph::new(Text::styled(
        "Brainfuck interpreter",
        Style::default().fg(Color::Green),
    ))
    .block(title_block)
    .centered();
    frame.render_widget(title, chunks[0]);

    // Current input
    let input_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());
    let input = Paragraph::new(Text::styled(
        &app.program,
        Style::default().fg(Color::White),
    ))
    .block(input_block);
    frame.render_widget(input, chunks[1]);

    // Footer
    let navigation_text = match app.current_screen {
        CurrentScreen::Main => Span::styled("Main window", Style::default().fg(Color::DarkGray)),
        CurrentScreen::Editing => {
            if let Some(editor_mode) = app.editor_mode {
                match editor_mode {
                    EditorMode::Normal => {
                        Span::styled("Input (Normal)", Style::default().fg(Color::LightCyan))
                    }
                    EditorMode::Insert => {
                        Span::styled("Input (Inserting)", Style::default().fg(Color::LightGreen))
                    }
                }
            } else {
                unreachable!()
            }
        }
        CurrentScreen::Running => {
            if let Some(running_mode) = app.running_mode {
                match running_mode {
                    RunningMode::StepByStep => {
                        Span::styled("Running (step by step)", Style::default().fg(Color::Green))
                    }
                    RunningMode::OneShot => {
                        Span::styled("Running (one shot)", Style::default().fg(Color::Green))
                    }
                }
            } else {
                Span::styled(
                    "Running (choose mode)",
                    Style::default().fg(Color::LightYellow),
                )
            }
        }
        CurrentScreen::Exiting => Span::styled("Exiting", Style::default().fg(Color::Red)),
    };
    let mode_footer =
        Paragraph::new(Line::from(navigation_text)).block(Block::default().borders(Borders::ALL));

    let footer_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);
    frame.render_widget(mode_footer, footer_layout[0]);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
