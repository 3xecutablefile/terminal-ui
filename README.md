# eDEX Native (Rust)

GPU-accelerated remake of the original eDEX-UI.  It launches your **real host shell** through a PTY/ConPTY backend and renders a neon dashboard with Rust, `winit`, and `wgpu`.

> ⚡ Early preview: basic terminal, theme switcher and CPU/RAM panels work.  Packaging and input polish are ongoing.

## Table of Contents
- [Features](#features)
- [Quick Start](#quick-start)
  - [Prebuilt Binaries](#prebuilt-binaries)
  - [Build from Source](#build-from-source)
- [Running](#running)
- [Configuration](#configuration)
  - [Themes](#themes)
  - [Keybinds](#keybinds)
- [Development](#development)
- [License](#license)

## Features
- **Real shell** via `portable-pty`/ConPTY – anything on your `$PATH` works (`git`, `vim`, `tmux`, ...).
- **GPU rendering** with `winit` + `wgpu`.
- **ANSI/UTF-8 terminal emulation** built on `vte`.
- **Theme system** with runtime switcher (`Ctrl/Cmd + Shift + T`).
- **System panels** for CPU and memory, sampled on a throttled cadence.

## Quick Start
### Prebuilt Binaries
Download the archive for your platform from [Releases](https://github.com/3xecutablefile/terminal-ui/releases) and extract it.

| OS    | Arch   | Artifact example      |
|-------|--------|----------------------|
| Linux | x86_64 | `app-linux-x64.zip`  |
| macOS | x86_64 | `app-macos-x64.zip` |

### Build from Source
Requirements:
- Rust toolchain (pinned by `rust-toolchain.toml`, ≥1.79)
- Vulkan (Linux) or Metal (macOS) capable GPU and drivers

```bash
# clone
git clone https://github.com/3xecutablefile/terminal-ui.git
cd terminal-ui

# build the native workspace
cargo build --release --manifest-path native/Cargo.toml -p app

# optional: symlink the binary
sudo ln -sf "$(pwd)/native/target/release/app" /usr/local/bin/terminal-ui
```

### Clear old installation
If you have a previous build installed, remove its binary and config before installing the new one:

```bash
sudo rm -f /usr/local/bin/terminal-ui       # old symlink or binary
rm -rf ~/.config/edex-native                # Linux/macOS config and themes
rm -rf ~/Library/Application\ Support/edex-native  # macOS alt config path
```

Then follow the steps above to install the latest version.

## Running
```bash
# run with defaults
app

# choose a theme on startup
app --theme "Tron Neon"

# specify initial terminal size
app --cols 120 --rows 36
```
Inside the window:
- **F1** runs `nmap --version` (diagnostic shortcut).
- **Ctrl/Cmd + Shift + T** opens the theme switcher.

## Configuration
Default config file:
- Linux/macOS: `~/.config/edex-native/config.toml`
- macOS Alt: `~/Library/Application Support/edex-native/config.toml`

```toml
[appearance]
theme = "Tron Neon"
font_family = "JetBrains Mono"
font_size = 16

[shell]
login = true
```

### Themes
TOML theme files are loaded from:
1. `~/.config/edex-native/themes/*.toml`
2. `native/app/assets/themes/*.toml` (bundled)

Example:
```toml
[terminal]
foreground = "#D6EFFF"
background = "#07121A"
cursor     = "#66FCF1"

[effects]
grid_color       = "rgba(0,229,255,0.23)"
scanline_opacity = 0.06
```

### Keybinds
| Action               | Shortcut                |
|----------------------|-------------------------|
| Theme switcher       | `Ctrl/Cmd + Shift + T`  |
| Copy / Paste         | Standard OS shortcuts   |

## Development
- Workspace is under `native/`
- Run checks before submitting PRs:
  ```bash
  cargo fmt --all --manifest-path native/Cargo.toml -- --check
  cargo clippy --workspace --manifest-path native/Cargo.toml -- -D warnings
  cargo test --workspace --manifest-path native/Cargo.toml
  ```
- CI builds Linux and macOS x86_64 targets and uploads zipped binaries.

## License
- Rust code in this repo: MIT OR Apache-2.0
- Legacy eDEX assets (if reused): GPL-3.0
