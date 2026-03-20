#[derive(Clone, Debug, Default)]
pub struct App {
    pub program: String,
    pub current_screen: CurrentScreen,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CurrentScreen {
    #[default]
    Main,
    Editing,
    Running,
}
