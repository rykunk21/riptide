pub enum TUI {
    Idle,
    RunningCommand(String),
    LaunchingCLI,
}

impl TUI {
    pub fn open(self: &mut TUI) {
        todo!();
    }
}
