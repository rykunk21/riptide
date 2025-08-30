### Actor Definitions
#### User
* State:
  - LoggedIn: The user is logged in to the system.
  - NotLoggedIn: The user is not logged in to the system.

#### CLI
* Actions:
  - LaunchRiptide: The CLI launches the Riptide application.
  - ListSubmodules: The CLI lists available submodules.
  - InstallSubmodule: The CLI installs a submodule.
  - RunModuleDirectly: The CLI runs a submodule directly without TUI.
  - ViewLogs: The CLI views logs in plain mode.

#### TUI
* Actions:
  - NavigateToolRegistry: The TUI navigates to the tool registry section.
  - NavigateWorkflowVisualizer: The TUI navigates to the workflow visualizer section.
  - NavigateLogsAndMetricsViewer: The TUI navigates to the logs and metrics viewer section.
  - DisplayLiveUpdates: The TUI displays live updates from running pipelines.
  - SplitPanes: The TUI splits panes vertically or horizontally.

#### View
* State:
    - Workflow
    - Logs
    - Help
