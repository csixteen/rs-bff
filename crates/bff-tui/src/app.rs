use std::sync::{Arc, RwLock};

use bff_core::AbstractMachine;

use crate::error::{Error, Result};

pub struct App {
    input: Arc<RwLock<String>>,
    output: Arc<RwLock<String>>,
    current_screen: CurrentScreen,
    editing_mode: EditingMode,
    running_mode: RunningMode,
    machine: Option<Arc<RwLock<AbstractMachine>>>,
}

impl App {
    pub fn new(input: Arc<RwLock<String>>, output: Arc<RwLock<String>>) -> Self {
        Self {
            input,
            output,
            current_screen: Default::default(),
            editing_mode: Default::default(),
            running_mode: Default::default(),
            machine: None,
        }
    }

    pub fn run_program(&self) -> Result<()> {
        if let Some(machine) = &self.machine {
            match self.running_mode {
                RunningMode::StepByStep => machine.try_write()?.step()?,
                RunningMode::OneShot => machine.try_write()?.run()?,
            }
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

    pub fn into_running_mode(self, running_mode: RunningMode) -> Self {
        Self {
            current_screen: CurrentScreen::Running,
            running_mode,
            ..self
        }
    }

    pub fn current_screen(&self) -> CurrentScreen {
        self.current_screen
    }

    pub fn editing_mode(&self) -> EditingMode {
        self.editing_mode
    }

    pub fn running_mode(&self) -> RunningMode {
        self.running_mode
    }

    pub fn input(&self) -> Result<String> {
        Ok(self.input.try_read()?.to_owned())
    }

    pub fn push_char(&self, c: char) -> Result<()> {
        self.input.try_write()?.push(c);

        Ok(())
    }

    pub fn pop_char(&self) -> Result<Option<char>> {
        Ok(self.input.try_write()?.pop())
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
