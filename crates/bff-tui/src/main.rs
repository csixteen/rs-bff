mod app;
mod error;
mod ui;
mod util;

use std::{
    io,
    sync::{Arc, RwLock},
};

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
    app::{App, CurrentScreen, EditingMode, RunningMode},
    error::{Error, Result},
    ui::ui,
};

fn main() -> Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create the app and run it
    run(&mut terminal)?;

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

fn run<B: Backend>(terminal: &mut Terminal<B>) -> Result<()>
where
    Error: From<B::Error>,
{
    let input = Arc::new(RwLock::new(Vec::new()));
    let output = Arc::new(RwLock::new(Vec::new()));
    let mut app = App::new(Arc::clone(&input), Arc::clone(&output));

    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // skip events that are not KeyEventKind::Press
                continue;
            }

            match app.current_screen() {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('e') => app = app.into_editing_mode(Default::default()),
                    KeyCode::Char('r') => app = app.into_running_mode(Default::default())?,
                    KeyCode::Char('q') => app = app.with_current_screen(CurrentScreen::Exiting),
                    _ => (),
                },
                CurrentScreen::Editing => match app.editing_mode() {
                    EditingMode::Normal => match key.code {
                        KeyCode::Esc => app = app.with_current_screen(CurrentScreen::Main),
                        KeyCode::Char('i') => app = app.into_editing_mode(EditingMode::Insert),
                        KeyCode::Char('q') => app = app.with_current_screen(CurrentScreen::Exiting),
                        _ => (),
                    },
                    EditingMode::Insert => match key.code {
                        KeyCode::Esc => app = app.with_current_screen(CurrentScreen::Main),
                        KeyCode::Char(value) => app = app.push_char(value)?,
                        KeyCode::Backspace => {
                            app = app.pop_char()?;
                        }
                        _ => (),
                    },
                },
                CurrentScreen::Running => match key.code {
                    KeyCode::Enter => app.run_program()?,
                    KeyCode::Char('o') => app = app.into_running_mode(RunningMode::OneShot)?,
                    KeyCode::Char('s') => app = app.into_running_mode(RunningMode::StepByStep)?,
                    KeyCode::Esc => app = app.with_current_screen(CurrentScreen::Main),
                    _ => (),
                },
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Esc => app = app.with_current_screen(CurrentScreen::Main),
                    _ => (),
                },
            }
        }
    }
}
