use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, RwLock},
};

use bff_core::{AbstractMachine, ReadOne};
use ratatui::text::Line;

use crate::error::{Error, Result};

pub struct App<'a> {
    input: Arc<RwLock<Vec<u8>>>,
    output: Arc<RwLock<Vec<u8>>>,
    cursor_index: usize,
    current_screen: CurrentScreen,
    editing_mode: EditingMode,
    running_mode: RunningMode,
    machine: Option<Rc<RefCell<AbstractMachine<'a>>>>,
}

impl<'a> App<'a> {
    pub fn new(input: Arc<RwLock<Vec<u8>>>, output: Arc<RwLock<Vec<u8>>>) -> Self {
        Self {
            input,
            output,
            cursor_index: 0,
            current_screen: Default::default(),
            editing_mode: Default::default(),
            running_mode: Default::default(),
            machine: None,
        }
    }

    pub fn run_program(&self) -> Result<()> {
        if let Some(machine) = &self.machine {
            match self.running_mode {
                RunningMode::StepByStep => machine.borrow_mut().step()?,
                RunningMode::OneShot => machine.borrow_mut().run()?,
            }

            return Ok(());
        }

        Err(Error::AbstractMachineMissing)
    }

    pub fn with_current_screen(self, current_screen: CurrentScreen) -> Self {
        Self {
            current_screen,
            ..self
        }
    }

    pub fn into_editing_mode(self, editing_mode: EditingMode) -> Self {
        Self {
            current_screen: CurrentScreen::Editing,
            editing_mode,
            ..self
        }
    }

    pub fn into_running_mode(self, running_mode: RunningMode) -> Result<Self> {
        let machine = Some(match self.machine {
            Some(m) => m,
            None => {
                let program: Arc<[u8]> = self.input.try_read()?.as_slice().into();
                let reader = Arc::new(RwLock::new(FakeReader));
                Rc::new(RefCell::new(AbstractMachine::new(
                    program,
                    reader,
                    self.output.clone(),
                )))
            }
        });

        Ok(Self {
            current_screen: CurrentScreen::Running,
            running_mode,
            machine,
            ..self
        })
    }

    #[inline]
    pub fn current_screen(&self) -> CurrentScreen {
        self.current_screen
    }

    #[inline]
    pub fn editing_mode(&self) -> EditingMode {
        self.editing_mode
    }

    #[inline]
    pub fn running_mode(&self) -> RunningMode {
        self.running_mode
    }

    #[inline]
    pub fn cursor_index(&self) -> usize {
        self.cursor_index
    }

    #[inline]
    pub fn input(&self) -> Result<Vec<u8>> {
        Ok(self.input.try_read()?.to_owned())
    }

    #[inline]
    pub fn input_to_lines(&self, line_width: usize) -> Result<Vec<Line<'_>>> {
        let mut lines = Vec::new();

        for chunk in self.input()?.chunks(line_width) {
            for parts in chunk.split(|&b| b == b'\n') {
                let s = String::from_utf8_lossy(parts).into_owned();
                lines.push(Line::from(s));
            }
        }

        Ok(lines)
    }

    pub fn push_char(self, c: char) -> Result<Self> {
        self.input.try_write()?.push(c as u8);

        Ok(Self {
            cursor_index: self.cursor_index.saturating_add(1),
            ..self
        })
    }

    pub fn pop_char(self) -> Result<Self> {
        let _ = self.input.try_write()?.pop();

        Ok(Self {
            cursor_index: self.cursor_index.saturating_sub(1),
            ..self
        })
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CurrentScreen {
    #[default]
    Main,
    Editing,
    Running,
    Exiting,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum EditingMode {
    #[default]
    Normal,
    Insert,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RunningMode {
    StepByStep,
    #[default]
    OneShot,
}

struct FakeReader;

impl ReadOne for FakeReader {
    fn read_one(&mut self) -> bff_core::Result<u8> {
        todo!()
    }
}
