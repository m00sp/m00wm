# m00wm

[![en](https://img.shields.io/badge/English-EB5406?style=for-the-badge&logoColor=white&logo=DocuSign)](https://github.com/m00sp/m00wm/blob/main/README.md) [![es](https://img.shields.io/badge/Español-380000?style=for-the-badge&logoColor=white&logo=DocuSign)](https://github.com/m00sp/m00wm/blob/main/README.es.md) [![br](https://img.shields.io/badge/Português-380000?style=for-the-badge&logoColor=white&logo=DocuSign)](https://github.com/m00sp/m00wm/blob/main/README.pt-BR.md)
A tiling X11 window manager written in Rust, built from scratch using the awesome [penrose](https://github.com/sminez/penrose) crate.

This is a learning project that builds a fully featured window manager incrementally. Please ensure you have an alternative desktop environment available in case anything breaks during testing.

> **Special thanks** to [sminez](https://github.com/sminez) for the excellent penrose crate and comprehensive documentation that made this project possible!

## Overview

m00wm is a minimalist tiling window manager that brings simplicity and control to your X11 desktop. Built with Rust, it prioritizes performance, stability, and hackability.

**Follow the development:**
- Watch the creation process on [YouTube](https://www.youtube.com/playlist?list=PLy2HjaQiG8lOxCKzuWKfmmXov4iEVOGOF)
- Check [progress-so-far.md](./progress-so-far.md) for a changelog of implemented features

## Features

- **Tiling layouts** – Efficient workspace management with multiple layout options
- **Customizable keybindings** – Fully configurable keyboard shortcuts
- **Status bar** – Built-in status bar with system information display
- **Multi-workspace support** – Organize windows across virtual workspaces
- **Floating window management** – Support for floating windows when needed
- **Dynamic spacing** – Configurable gaps between windows

## Tech Stack

- **Language:** [Rust](https://rust-lang.org)
- **Window Manager Framework:** [Penrose](https://github.com/sminez/penrose)
- **X11 Protocol:** x11rb
- **Logging:** tracing-subscriber

## Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- X11 development headers
- A display manager supporting custom desktop sessions
- An alternative desktop environment for fallback (recommended during development)

## Installation

> **NOTE:** Read the [Makefile](./Makefile) before installing to understand what will be executed. There's nothing harmful, but always review what you're running with `sudo`!

1. Clone the repository:
```bash
git clone https://github.com/m00sp/m00wm.git
cd m00wm
```

2. Review and customize key bindings in `src/main.rs` if needed (default uses `st` terminal and `dmenu_run`):
```bash
# Replace default terminal and launcher references
vim src/main.rs
```

3. Build and install:
```bash
make build && sudo make install
```

The window manager will be available as a desktop session in your display manager. Select "m00wm" or "Plasma m00wm" at login.

## Building & Development

Build in release mode:
```bash
make build
```

Test in a nested X session:
```bash
make test
```

Uninstall:
```bash
sudo make uninstall
```

## Project Structure

```
src/
├── main.rs           # Entry point, keybindings, and window manager config
├── lib.rs            # Library exports and constants
├── bar.rs            # Status bar implementation
├── layouts.rs        # Tiling layout definitions
examples/             # Example configurations
config/               # Desktop session files
scripts/              # Build and test utilities
```

## Configuration

Key configurations are defined in `src/main.rs`:
- **Keybindings** – Modify the keybinding map to customize shortcuts
- **Layouts** – Add or adjust tiling layouts in `src/layouts.rs`
- **Colors & Fonts** – Theme options in constants at the top of `main.rs`
- **Status Bar** – Customize status bar display in `src/bar.rs`

## Getting Help

- Check [progress-so-far.md](./progress-so-far.md) for implementation status
- Review the [penrose documentation](https://github.com/sminez/penrose)
- Explore examples in the `examples/` directory

## Acknowledgments

- **sminez** for the [penrose](https://github.com/sminez/penrose) window manager framework and excellent documentation

