#![feature(if_let_guard)]

mod app;
mod ui;

use std::{error::Error, io};

use ratatui::{
    Terminal,
    backend::Backend,
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    prelude::CrosstermBackend,
};

use self::{
    app::{App, CurrentScreen, EditorMode},
    ui::ui,
};

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create the app and run it
    let mut app = App::default();
    run_app(&mut terminal, &mut app)?;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()>
where
    io::Error: From<B::Error>,
{
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // skip events that are not KeyEventKind::Press
                continue;
            }

            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('e') => {
                        app.current_screen = CurrentScreen::Editing;
                        app.editor_mode = Some(Default::default());
                    }
                    KeyCode::Char('q') => app.current_screen = CurrentScreen::Exiting,
                    KeyCode::Char('r') => app.current_screen = CurrentScreen::Running,
                    KeyCode::Tab => app.next_screen(),
                    _ => (),
                },
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Tab => app.current_screen = app.current_screen.next(),
                    KeyCode::Esc => app.current_screen = CurrentScreen::Main,
                    KeyCode::Char('q') => return Ok(()),
                    _ => (),
                },
                // In theory, if we're on `Editing` screen, then `editor_mode` should always be
                // `Some(T)`.
                CurrentScreen::Editing if let Some(editor_mode) = app.editor_mode => {
                    match editor_mode {
                        EditorMode::Normal => match key.code {
                            KeyCode::Esc => {
                                app.current_screen = CurrentScreen::Main;
                                app.editor_mode = None;
                            }
                            KeyCode::Tab => {
                                app.current_screen = app.current_screen.next();
                                app.editor_mode = None;
                            }
                            KeyCode::Char('i') => app.editor_mode = Some(app::EditorMode::Insert),
                            _ => (),
                        },
                        EditorMode::Insert => match key.code {
                            KeyCode::Esc => app.editor_mode = Some(Default::default()),
                            KeyCode::Backspace => {
                                let _ = app.program.pop();
                            }
                            KeyCode::Char(value) => app.program.push(value),
                            _ => (),
                        },
                    }
                }
                CurrentScreen::Running => {
                    if let Some(_running_mode) = app.running_mode {
                        todo!()
                    } else {
                        todo!()
                    }
                }
                _ => (),
            }
        }
    }
}
