use bff_core::AbstractMachine;

#[derive(Default)]
pub struct App {
    pub program: String,
    pub output: String,
    pub current_screen: CurrentScreen,
    pub editor_mode: Option<EditorMode>,
    pub running_mode: Option<RunningMode>,
    pub machine: Option<AbstractMachine>,
}

impl App {
    pub fn next_screen(&mut self) {
        self.current_screen = self.current_screen.next();

        if matches!(self.current_screen, CurrentScreen::Editing) {
            self.editor_mode = Some(Default::default());
        }
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

impl CurrentScreen {
    pub fn next(&self) -> Self {
        match self {
            CurrentScreen::Main => Self::Editing,
            CurrentScreen::Editing => Self::Running,
            CurrentScreen::Running => Self::Exiting,
            CurrentScreen::Exiting => Self::Main,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum EditorMode {
    #[default]
    Normal,
    Insert,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunningMode {
    StepByStep,
    OneShot,
}
