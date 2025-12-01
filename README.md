# Shortcut Manager

A Rust library and tool to manage **keyboard shortcuts** across multiple IDEs (VSCode, JetBrains, etc.) with a **unified internal model**.

---

## Features

- Unified internal `Shortcut` model for different IDEs
- Convert **VSCode shortcuts** → internal model
- Convert internal model → **VSCode JSON** and **JetBrains XML**
- Extensible for future IDEs (Sublime, Emacs, Neovim, etc.)
- Supports context (`when`), sequences, and removal flags for JetBrains

---

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
quick-xml = "0.30" # if using JetBrains XML export
