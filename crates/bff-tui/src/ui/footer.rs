use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::{App, CurrentScreen, EditorMode, RunningMode};

pub fn render(frame: &mut Frame, rect: Rect, app: &App) {
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
                unreachable!("Editor screen always has an editor mode set.")
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
    let navigation_footer =
        Paragraph::new(Line::from(navigation_text)).block(Block::default().borders(Borders::ALL));

    let keys_hint_text = match app.current_screen {
        CurrentScreen::Main => Span::styled(
            "(e)dit | (r)un | (q)uit | TAB",
            Style::default().fg(Color::Red),
        ),
        CurrentScreen::Editing => todo!(),
        CurrentScreen::Running => todo!(),
        CurrentScreen::Exiting => todo!(),
    };
    let keys_hint_footer =
        Paragraph::new(Line::from(keys_hint_text)).block(Block::default().borders(Borders::ALL));

    let footer_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(rect);
    frame.render_widget(navigation_footer, footer_layout[0]);
    frame.render_widget(keys_hint_footer, footer_layout[1]);
}
