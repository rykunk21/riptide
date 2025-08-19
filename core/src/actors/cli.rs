#[derive(Debug, PartialEq, Eq)]
pub enum CLI {
    Idle,
    RunningCommand(String),
    LaunchingTUI,
}

impl CLI {
    pub fn start(self: &mut CLI) {
        todo!();
    }
}
