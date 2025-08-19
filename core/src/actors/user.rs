use crate::actors::{CLI, TUI, View};

#[derive(Debug, PartialEq, Eq)]
pub enum User {
    Idle,
    InCLI,
    InTUI,
    DefiningWorkflow,
    RunningWorkflow,
    ReviewingLogs,
}

impl User {
    pub fn launch_cli(&mut self, cli: &mut CLI) {
        todo!();
    }

    pub fn launch_tui(&mut self, cli: &CLI, tui: &mut TUI) {
        todo!();
    }
    pub fn define_workflow(&mut self, tui: &mut TUI) {
        todo!();
    }

    pub fn run_workflow(&mut self) {
        todo!();
    }
    pub fn view_logs(&mut self, tui: &mut TUI) {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::{automock, predicate::*};

    // Traits only for mocking in tests
    #[automock]
    trait CLI {
        fn start(&mut self);
        fn get_state(&self) -> UserCLIState;
    }

    #[automock]
    trait TUI {
        fn open(&mut self);
        fn set_view(&mut self, view: View);
    }

    // Helper enum for CLI state (mirrors CLI enum in prod)
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum UserCLIState {
        Idle,
        RunningCommand,
    }

    // Mock enum for Views (simplified)
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum View {
        Workflow,
        Logs,
    }

    #[test]
    fn test_launch_cli() {
        let mut user = User::Idle;

        let mut mock_cli = MockCLI::new();
        mock_cli.expect_start().times(1).return_const(());
        user.launch_cli(&mut mock_cli);

        assert_eq!(user, User::InCLI);
    }

    #[test]
    fn test_launch_tui_only_from_cli_and_running() {
        let mut user = User::Idle;
        let mut mock_cli = MockCLI::new();
        let mut mock_tui = MockTUI::new();

        // Should not open TUI if user not InCLI
        mock_tui.expect_open().times(0);
        mock_cli.expect_get_state().times(0);
        user.launch_tui_mock(&mock_cli, &mut mock_tui);
        assert_eq!(user, User::Idle);

        // Set user to InCLI but CLI is idle, TUI open should NOT be called
        user = User::InCLI;
        mock_cli
            .expect_get_state()
            .times(1)
            .return_const(UserCLIState::Idle);
        mock_tui.expect_open().times(0);
        user.launch_tui_mock(&mock_cli, &mut mock_tui);
        assert_eq!(user, User::InCLI);

        // CLI running, TUI should open
        user = User::InCLI;
        mock_cli
            .expect_get_state()
            .times(1)
            .return_const(UserCLIState::RunningCommand);
        mock_tui.expect_open().times(1).return_const(());
        user.launch_tui_mock(&mock_cli, &mut mock_tui);
        assert_eq!(user, User::InTUI);
    }

    #[test]
    fn test_define_workflow() {
        let mut user = User::InTUI;
        let mut mock_tui = MockTUI::new();

        mock_tui
            .expect_set_view()
            .with(eq(View::Workflow))
            .times(1)
            .return_const(());
        user.define_workflow_mock(&mut mock_tui);
        assert_eq!(user, User::DefiningWorkflow);
    }

    #[test]
    fn test_run_workflow() {
        let mut user = User::DefiningWorkflow;
        user.run_workflow_mock();
        assert_eq!(user, User::RunningWorkflow);

        let mut user2 = User::Idle;
        user2.run_workflow_mock();
        assert_ne!(user2, User::RunningWorkflow);
    }

    #[test]
    fn test_view_logs() {
        let mut mock_tui = MockTUI::new();

        mock_tui
            .expect_set_view()
            .with(eq(View::Logs))
            .times(2)
            .return_const(());

        let mut user = User::InTUI;
        user.view_logs_mock(&mut mock_tui);
        assert_eq!(user, User::ReviewingLogs);

        let mut user2 = User::DefiningWorkflow;
        user2.view_logs_mock(&mut mock_tui);
        assert_eq!(user2, User::ReviewingLogs);

        let mut user3 = User::Idle;
        user3.view_logs_mock(&mut mock_tui);
        assert_ne!(user3, User::ReviewingLogs);
    }
}
