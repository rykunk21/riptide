### Actors

# Riptide Actor Design (Core Set)

## 1. User
Represents the human interacting with Riptide.

### States (enum variants)
```rust
enum User {
    Idle,
    InCLI(CLI),
    InTUI(TUI),
    DefiningWorkflow,
    RunningWorkflow,
    ReviewingLogs,
}


| Action                      | Manipulates                   | Notes                          |
| --------------------------- | ----------------------------- | ------------------------------ |
| `launch_cli()`              | CLI                           | Starts the CLI interface.      |
| `launch_tui(cli: CLI)`      | TUI                           | Requests CLI to open TUI.      |
| `define_workflow(tui: TUI)` | View (Workflow View)          | Adds/edit workflow stages.     |
| `run_workflow(tui: TUI)`    | WorkflowOrchestrator (future) | Starts pipeline execution.     |
| `view_logs(tui: TUI)`       | View (Logs View)              | Switches to logs/metrics view. |


enum CLI {
    Idle,
    RunningCommand(String), // e.g., "install submodule"
    LaunchingTUI,
}


| Action                                            | Manipulates  | Notes                                 |
| ------------------------------------------------- | ------------ | ------------------------------------- |
| `parse_command(user: &User)`                      | CLI          | Reads user input into internal state. |
| `install_submodule(registry: &mut ToolRegistry)`  | ToolRegistry | Adds submodule to installed list.     |
| `list_submodules(registry: &ToolRegistry)`        | User         | Outputs available tools to user.      |
| `launch_tui()`                                    | TUI          | Creates new TUI actor instance.       |
| `run_submodule_direct(submodule: &mut Submodule)` | Submodule    | Executes without TUI.                 |
| `view_logs_plain(logger: &Logger)`                |              |                                       |



enum TUI {
    Idle,
    Viewing(View),
    ExecutingWorkflow,
}


| Action                                      | Manipulates      | Notes                          |
| ------------------------------------------- | ---------------- | ------------------------------ |
| `navigate_to(view: View)`                   | TUI              | Switches current view.         |
| `render_current_view()`                     | View             | Draws active view.             |
| `execute_workflow(workflow: &mut Workflow)` | Workflow         | Runs pipeline stages.          |
| `display_logs(logger: &Logger)`             | View (Logs View) | Pulls from Logger for display. |



enum View {
    ToolRegistry,
    WorkflowVisualizer,
    LogsViewer,
    MetricsViewer,
    ConfigEditor,
}


| Action                                   | Manipulates | Notes                                        |
| ---------------------------------------- | ----------- | -------------------------------------------- |
| `show_registry(registry: &ToolRegistry)` | User        | Displays all available/installed submodules. |
| `show_workflow(workflow: &Workflow)`     | User        | Shows workflow layout.                       |
| `show_logs(logger: &Logger)`             | User        | Displays live or past logs.                  |
| `show_metrics(metrics: &MetricsStore)`   | User        | Displays charts/tables of performance.       |
| `edit_config(config: &mut Config)`       | User        | Allows config changes.                       |
