# eDEX Native (Rust) — Real Shell • GPU UI

A native Rust reimagining of the classic eDEX-UI: flashy, themeable terminal dashboard that launches your **real host shell** (bash/zsh/fish on Unix) through a PTY backend — no browser, no Electron.

> ⚡ Status: early preview. Terminal, theme switcher, CPU/RAM panels, and ambient effects hooks are in. Packaging & input polish are ongoing.

---

## ✨ Features

- **Real OS shell** via `portable-pty` — anything on your `$PATH` works (`git`, `nmap`, `vim`, `tmux`, etc.).
- **GPU rendering** (`winit` + `wgpu`) with theme-driven neon UI.
- **Terminal emulation** (Alacritty stack), DPI-aware sizing (in progress).
- **Runtime theme switcher** (`Ctrl/Cmd + Shift + T`), TOML themes (dark/light included).
- **System panels** (CPU / RAM) on a decoupled cadence (250–500 ms).
- **Ambient effects** (neon grid + scanlines) from theme settings.

---

## 📦 Prebuilt Downloads

| OS    | Arch   | Artifact (example)  |
|-------|--------|---------------------|
| Linux | x86_64 | `app-linux-x64.zip` |
| macOS | x86_64 | `app-macos-x64.zip` |

> If you’re building from source, see **Build from Source** below.

---

## Platform support

- ✅ Linux (x86_64)
- ✅ macOS (x86_64 / Intel)
- 🚫 Windows (not supported in this fork)

---

## 💾 Install

Download the archive for your platform from the table above and extract it.

### Linux

```bash
tar -xf app-linux-*.tar.gz   # or unzip app-linux-*.zip
./app                        # run the binary
````

### macOS

```bash
unzip app-macos-*.zip
./app-macos-x64/app
```

---

## 🛠 Build from Source

### Prereqs

* **Rust**: pinned via `rust-toolchain.toml` (≥ 1.79.0)
* **GPU**: Vulkan (Linux) or Metal (macOS)
* **Linux**: Vulkan loader (`libvulkan1`) + GPU drivers (Mesa/NVIDIA)
* **macOS**: 12+ recommended

### Clone & build (macOS/Linux)

```bash
git clone https://github.com/3xecutablefile/terminal-ui.git
cd terminal-ui
cargo build --release -p app
sudo ln -sf "$(pwd)/target/release/app" /usr/local/bin/terminal-ui
# verify the shortcut
terminal-ui --version
```

### Quick build

```bash
# in repo root (or use --manifest-path native/Cargo.toml)
cargo build --release -p app
./target/release/app   # run (path varies per OS)
```

### Cross-platform targets (examples)

```bash
# Linux x64
cargo build --release -p app --target x86_64-unknown-linux-gnu

# macOS x64
cargo build --release -p app --target x86_64-apple-darwin
```

---

## ▶️ Run

The app launches your host shell inside a PTY.

```bash
# Run with defaults
app

# Choose theme on startup
app --theme "Tron Neon"

# Set initial size (columns/rows) if desired
app --cols 120 --rows 36
```

Inside the window:

* Press **F1** to run `nmap --version` (diagnostic shortcut, optional).
* Press **Ctrl/Cmd + Shift + T** to open the **theme switcher**.

---

## 🎨 Themes

* Themes are TOML files.
* Bundled: `Tron Neon` (dark), `Mono Light`.
* Locations searched (in order):

  1. `~/.config/edex-native/themes/*.toml`
  2. `native/app/assets/themes/*.toml` (bundled)

Example (`tron.toml`):

```toml
[terminal]
foreground = "#D6EFFF"
background = "#07121A"
cursor     = "#66FCF1"
# … plus the 16-color ANSI palette …

[ui]
panel_bg     = "rgba(10,18,28,0.85)"
panel_border = "#0EE7FF"
text         = "#CFE9FF"
accent       = "#00E5FF"

[effects]
grid_color        = "rgba(0,229,255,0.23)"
grid_spacing      = 28
scanline_opacity  = 0.06
```

---

## ⚙️ Config

Default config path:

* Linux/macOS: `~/.config/edex-native/config.toml`
* macOS: `~/Library/Application Support/edex-native/config.toml`

Example:

```toml
[appearance]
theme = "Tron Neon"
font_family = "JetBrains Mono"
font_size = 16

[render]
backend = "auto"         # auto|vulkan|metal
grid_spacing = 28
scanline_opacity = 0.06

[shell]
login = true
```

---

## ⌨️ Keybinds (default)

* **Theme switcher**: `Ctrl/Cmd + Shift + T`
* **Copy/Paste**: native OS shortcuts (plus OSC-52 for remote apps; size-capped)
* **Mouse**: SGR mouse reporting (when enabled in terminal apps; in progress)
* **IME**: winit composition events (in progress)

---

## 🧪 Smoke Tests

Inside the app:

```sh
whoami
uname -a
git --version
nmap --version      # if installed
vim; htop; tmux     # should render & respond (alt-screen)
printf '😀 測試 ä̈ café\n'
```

Resize the window → the grid should reflow without drift.

---

## 🧰 Development

* PTY daemon (`native/ptyd`) speaks NDJSON:

  * Input frame: `{"t":"i","data":"<base64-bytes>"}`
  * Resize: `{"t":"r","cols":120,"rows":40}`
  * Signal: `{"t":"s","sig":"INT"}`
  * Output: `{"t":"o","data":"<base64-bytes>","seq":N}`
  * Exit: `{"t":"x","code":0}`
* Native app (`native/app`) renders with `wgpu`, feeds PTY → emulator → GPU.

### CI (GitHub Actions)

* Matrix builds: Linux (x64) and macOS (x64)
* Steps: `fmt`, `clippy -D warnings`, `test`, `build`
* Artifacts include binary + `assets/` (themes, shaders)

---

## 🐛 Troubleshooting

* **Linux/Wayland**: ensure Vulkan loader (`libvulkan1`) and GPU driver installed.
* **No colors/Truecolor**: verify `TERM=xterm-256color`.
* **Huge output (e.g., `yes`)**: PTY → UI buffer is capped; rendering may rate-limit.

---

## 🙌 Credits

* **Fork maintainer**: **@3xecutablefile** — project direction, migration plan, native UI, and theme system.
* Inspired by the original **eDEX-UI** concept. If you reuse original eDEX assets/themes, respect their **GPL-3.0** license.
* This Rust rewrite’s code is licensed as noted below.

---

## 📜 License

* **Rust code (this repository)**: MIT or Apache-2.0 (choose one).
* **Legacy eDEX assets** (if reused): GPL-3.0. Mixing GPL assets imposes GPL terms on the combined distribution.

---

## 📬 Contributing

PRs and issues welcome!

* Run `cargo fmt`, `cargo clippy -D warnings`, `cargo test` before pushing.
* Add screenshots/gifs for UI PRs (themes/effects).
* Keep panels on a throttled update cadence; never block the terminal render path.

```
```
