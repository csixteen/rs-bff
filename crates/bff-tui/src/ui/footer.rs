use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::{App, CurrentScreen, EditingMode, RunningMode};

pub fn render(frame: &mut Frame, rect: Rect, app: &App) {
    let navigation_text = match app.current_screen() {
        CurrentScreen::Main => Span::styled("Main window", Style::default().fg(Color::White)),
        CurrentScreen::Editing => match app.editing_mode() {
            EditingMode::Normal => {
                Span::styled("Input (Normal)", Style::default().fg(Color::LightCyan))
            }
            EditingMode::Insert => {
                Span::styled("Input (Inserting)", Style::default().fg(Color::LightGreen))
            }
        },
        CurrentScreen::Running => match app.running_mode() {
            RunningMode::StepByStep => {
                Span::styled("Running (step by step)", Style::default().fg(Color::Green))
            }
            RunningMode::OneShot => {
                Span::styled("Running (one shot)", Style::default().fg(Color::Green))
            }
        },
        CurrentScreen::Exiting => Span::styled("Exiting", Style::default().fg(Color::LightRed)),
    };
    let navigation_footer = Paragraph::new(Line::from(navigation_text))
        .block(Block::default().borders(Borders::ALL))
        .centered();

    let keys_hint_text = match app.current_screen() {
        CurrentScreen::Main => {
            Span::styled("(e)dit | (r)un | (q)uit", Style::default().fg(Color::White))
        }
        CurrentScreen::Editing => match app.editing_mode() {
            EditingMode::Normal => Span::styled(
                "ESC (return to Main) | (i)nsert ",
                Style::default().fg(Color::Cyan),
            ),
            EditingMode::Insert => {
                Span::styled("ESC (Normal mode)", Style::default().fg(Color::Green))
            }
        },
        CurrentScreen::Running => match app.running_mode() {
            RunningMode::StepByStep => {
                Span::styled("Enter (step)", Style::default().fg(Color::Green))
            }
            RunningMode::OneShot => Span::styled("Enter (run)", Style::default().fg(Color::Green)),
        },
        CurrentScreen::Exiting => {
            Span::styled("ESC (go to Main) | (q)uit", Style::default().fg(Color::Red))
        }
    };
    let keys_hint_footer = Paragraph::new(Line::from(keys_hint_text))
        .block(Block::default().borders(Borders::ALL))
        .centered();

    let footer_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(rect);
    frame.render_widget(navigation_footer, footer_layout[0]);
    frame.render_widget(keys_hint_footer, footer_layout[1]);
}
