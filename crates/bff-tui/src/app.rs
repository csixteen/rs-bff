use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, RwLock},
};

use bff_core::AbstractMachine;
use ratatui::{text::Line, widgets::ScrollbarState};

use crate::error::{Error, Result};

pub struct App<'a> {
    output: Arc<RwLock<Vec<u8>>>,
    program: &'a [u8],
    horizontal_scroll: usize,
    horizontal_scroll_state: ScrollbarState,
    vertical_scroll: usize,
    vertical_scroll_state: ScrollbarState,
    current_screen: CurrentScreen,
    running_mode: RunningMode,
    machine: Rc<RefCell<AbstractMachine<'a>>>,
}

impl<'a> App<'a> {
    pub fn new(
        output: Arc<RwLock<Vec<u8>>>,
        program: &'a [u8],
        machine: Rc<RefCell<AbstractMachine<'a>>>,
    ) -> Self {
        Self {
            output,
            program,
            horizontal_scroll: 0,
            horizontal_scroll_state: Default::default(),
            vertical_scroll: 0,
            vertical_scroll_state: Default::default(),
            current_screen: Default::default(),
            running_mode: Default::default(),
            machine,
        }
    }

    pub fn run_program(&self) -> Result<()> {
        match self.running_mode {
            RunningMode::StepByStep => self.machine.borrow_mut().step()?,
            RunningMode::OneShot => self.machine.borrow_mut().run()?,
        }

        Ok(())
    }

    pub fn with_current_screen(self, current_screen: CurrentScreen) -> Self {
        Self {
            current_screen,
            ..self
        }
    }

    #[inline]
    pub fn program_to_lines(&self, wrap: WrapLine) -> Vec<Line<'_>> {
        bytes_to_lines(self.program, wrap)
    }

    #[inline]
    pub fn output(&self) -> Result<Vec<u8>> {
        Ok(self.output.try_read()?.iter().cloned().collect())
    }

    #[inline]
    pub fn debug_info(&self) -> Vec<Line<'_>> {
        let di = self.machine.borrow().to_debug_info();

        vec![
            format!("Data pointer: {:#x}", di.data_pointer).into(),
            format!("Current cell: {:#x}", di.current_cell).into(),
            format!("Instruction pointer: {:#x}", di.instruction_pointer).into(),
            format!(
                "Current instruction: {:#x} ({c})",
                di.current_instruction,
                c = di.current_instruction as char
            )
            .into(),
        ]
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
    pub fn into_running_mode(self, running_mode: RunningMode) -> Self {
        Self {
            current_screen: CurrentScreen::Running,
            running_mode,
            ..self
        }
    }

    pub fn restart(&self) -> Result<()> {
        self.clear_output()?;
        self.machine
            .try_borrow_mut()
            .map_err(|_| Error::Lock)?
            .restart();

        Ok(())
    }

    #[inline]
    pub fn clear_output(&self) -> Result<()> {
        self.output.try_write().map_err(|_| Error::Lock)?.clear();

        Ok(())
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
    pub fn vertical_scroll(&self) -> usize {
        self.vertical_scroll
    }

    #[inline]
    pub fn vertical_scroll_state_mut(&mut self) -> &mut ScrollbarState {
        &mut self.vertical_scroll_state
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum WrapLine {
    #[default]
    Never,
    Width(usize),
}

pub fn bytes_to_lines(xs: &[u8], wrap: WrapLine) -> Vec<Line<'_>> {
    #[inline]
    fn break_lines(xs: &[u8]) -> Vec<Line<'_>> {
        let mut res = Vec::new();

        for line in xs.split(|&b| b == b'\n') {
            res.push(String::from_utf8_lossy(line).into_owned().into());
        }
        res
    }

    match wrap {
        WrapLine::Never => break_lines(xs),
        WrapLine::Width(width) => {
            let mut res = Vec::new();

            for chunk in xs.chunks(width) {
                res.extend(break_lines(chunk));
            }

            res
        }
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
