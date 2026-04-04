use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, RwLock},
};

use bff_core::{AbstractMachine, ReadOne};
use ratatui::{text::Line, widgets::ScrollbarState};

use crate::error::{Error, Result};

pub struct App<'a> {
    input: Arc<RwLock<Vec<u8>>>,
    output: Arc<RwLock<Vec<u8>>>,
    horizontal_scroll: usize,
    horizontal_scroll_state: ScrollbarState,
    vertical_scroll: usize,
    vertical_scroll_state: ScrollbarState,
    current_screen: CurrentScreen,
    running_mode: RunningMode,
    machine: Option<Rc<RefCell<AbstractMachine<'a>>>>,
}

impl<'a> App<'a> {
    pub fn new(input: Arc<RwLock<Vec<u8>>>, output: Arc<RwLock<Vec<u8>>>) -> Self {
        Self {
            input,
            output,
            horizontal_scroll: 0,
            horizontal_scroll_state: Default::default(),
            vertical_scroll: 0,
            vertical_scroll_state: Default::default(),
            current_screen: Default::default(),
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
                )?))
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
    pub fn running_mode(&self) -> RunningMode {
        self.running_mode
    }

    #[inline]
    pub fn scroll_down(self) -> Self {
        let vertical_scroll = self.vertical_scroll.saturating_add(1);

        Self {
            vertical_scroll,
            vertical_scroll_state: self.vertical_scroll_state.position(vertical_scroll),
            ..self
        }
    }

    #[inline]
    pub fn scroll_up(self) -> Self {
        let vertical_scroll = self.vertical_scroll.saturating_sub(1);

        Self {
            vertical_scroll,
            vertical_scroll_state: self.vertical_scroll_state.position(vertical_scroll),
            ..self
        }
    }

    #[inline]
    pub fn scroll_left(self) -> Self {
        let horizontal_scroll = self.horizontal_scroll.saturating_sub(1);

        Self {
            horizontal_scroll,
            horizontal_scroll_state: self.horizontal_scroll_state.position(horizontal_scroll),
            ..self
        }
    }

    #[inline]
    pub fn scroll_right(self) -> Self {
        let horizontal_scroll = self.horizontal_scroll.saturating_add(1);

        Self {
            horizontal_scroll,
            horizontal_scroll_state: self.horizontal_scroll_state.position(horizontal_scroll),
            ..self
        }
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
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CurrentScreen {
    #[default]
    Main,
    Running,
    Exiting,
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
