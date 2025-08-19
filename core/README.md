# 🌊 Riptide

**Riptide** is a nautical-themed CLI + TUI toolkit that bundles together essential tools for managing end-to-end MLOps workflows — all from your terminal.

Inspired by tools like `lazydocker` and `lazygit`, Riptide offers a fluid, navigable interface for orchestrating complex machine learning pipelines with simplicity, visibility, and control.

---

## ⚓ What is Riptide?

Riptide is both a **tool manager** and **workflow visualizer** for machine learning operations. It brings together modular sub-tools (like `SAIL`, `SURF`, and `DOCK`) into a cohesive experience focused on:

- 🧰 **Bundling tools**: Manage CLI-based ML tools in one place  
- 🛠️ **Workflow control**: Execute, monitor, and debug ML stages interactively  
- 🌊 **TUI interface**: Visualize your models, datasets, logs, and pipeline stages from the terminal  
- ⚙️ **Modular design**: Add your own custom toolchains and workflows  

---

## 🧭 Features

- 📦 **Tool Orchestration**: Seamlessly link together CLI tools like data wranglers, model trainers, and evaluators  
- 🖥️ **TUI Dashboard**: Visual, scrollable views for each pipeline stage and component  
- 🚢 **ML Tool Registry**: Discover, install, and run supported submodules (e.g. `SAIL`, `DOCK`, `SURF`)  
- 🔍 **Live Monitoring**: View logs, results, metrics, and model state in real time  
- 🔗 **Extensible Hooks**: Bring in your own shell scripts or Rust binaries for custom stages  

---

## 🧱 Submodules (Available Tools)

| Tool     | Role                                                  |
|----------|-------------------------------------------------------|
| `DOCK`   | Data Organization and Cleaning Kit                    |
| `SAIL`   | Stats, Automation, and Inferencing Library            |
| `SURF`   | Simulated UI Rating Framework                         |
| `BUOY`   | Behavioral Uptime and Observabiilty Yield             |

---

## 🚀 Getting Started

```bash
cargo install riptide
riptide launch
```

From there, navigate the TUI to manage tools, set up pipelines, or visualize progress.

---

## 🐚 Philosophy

MLOps shouldn't require cloud dashboards or heavyweight IDEs. Riptide brings it all back to the terminal — where speed, composability, and control thrive.

---

## 🧩 Coming Soon

- Plugin system for custom tools  
- Remote deployment integration  
- YAML-based pipeline definitions  
- Native support for Hugging Face datasets + models  

---

## 📫 Contribute

Pull requests, issues, and ideas welcome. If you're building nautical-themed tools, we'd love to link up. Let's sail together.
