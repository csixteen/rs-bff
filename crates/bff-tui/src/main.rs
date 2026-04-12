mod app;
mod error;
mod ui;

use std::{
    cell::RefCell,
    fs, io,
    path::PathBuf,
    rc::Rc,
    sync::{Arc, RwLock},
};

use bff_core::{AbstractMachine, ReadOne};
use clap::{Parser, ValueHint};
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
    app::{App, CurrentScreen, RunningMode},
    error::{Error, Result},
    ui::ui,
};

/// Brainfuck debugger
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of memory cells that the abstract machine will operate on
    #[arg(short, long, default_value_t = AbstractMachine::DEFAULT_NUM_CELLS)]
    cells: usize,

    #[arg(short, long, value_hint = ValueHint::FilePath)]
    file: PathBuf,
}

fn main() -> Result<()> {
    let Args { cells, file } = Args::parse();
    let program = fs::read(file)?;

    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create the app and run it
    run(&mut terminal, program, cells)?;

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

fn run<B: Backend>(terminal: &mut Terminal<B>, program: Vec<u8>, cells: usize) -> Result<()>
where
    Error: From<B::Error>,
{
    let output = Arc::new(RwLock::new(Vec::new()));
    let machine = Rc::new(RefCell::new(
        AbstractMachine::new(
            program.as_slice().into(),
            Arc::new(RwLock::new(FakeReader)),
            output.clone(),
        )?
        .with_num_cells(cells),
    ));
    let mut app = App::new(Arc::clone(&output), &program, machine);

    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // skip events that are not KeyEventKind::Press
                continue;
            }

            match app.current_screen() {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('r') => app = app.into_running_mode(Default::default()),
                    KeyCode::Char('q') => app = app.with_current_screen(CurrentScreen::Exiting),
                    // scrolling keys
                    KeyCode::Char('h') => app = app.scroll_left(),
                    KeyCode::Char('j') => app = app.scroll_down(),
                    KeyCode::Char('k') => app = app.scroll_up(),
                    KeyCode::Char('l') => app = app.scroll_right(),
                    _ => (),
                },
                CurrentScreen::Running => match key.code {
                    KeyCode::Enter => app.run_program()?,
                    KeyCode::Char('o') => app = app.into_running_mode(RunningMode::OneShot),
                    KeyCode::Char('s') => app = app.into_running_mode(RunningMode::StepByStep),
                    KeyCode::Char('r') => app.restart()?,
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

struct FakeReader;

impl ReadOne for FakeReader {
    fn read_one(&mut self) -> bff_core::Result<u8> {
        todo!()
    }
}
