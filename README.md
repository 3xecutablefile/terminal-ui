# eDEX Native (Rust) — Real Shell • GPU UI

A native Rust reimagining of the classic eDEX-UI: flashy, themeable terminal dashboard that launches your **real host shell** (bash/zsh/fish on Unix, PowerShell/cmd on Windows) through a PTY/ConPTY backend — no browser, no Electron.

> ⚡ Status: early preview. Terminal, theme switcher, CPU/RAM panels, and ambient effects hooks are in. Packaging & input polish are ongoing.

---

## ✨ Features

- **Real OS shell** via `portable-pty` / ConPTY — anything on your `$PATH` works (`git`, `nmap`, `vim`, `tmux`, etc.).
- **GPU rendering** (`winit` + `wgpu`) with theme-driven neon UI.
- **Terminal emulation** (Alacritty stack), DPI-aware sizing (in progress).
- **Runtime theme switcher** (`Ctrl/Cmd + Shift + T`), TOML themes (dark/light included).
- **System panels** (CPU / RAM) on a decoupled cadence (250–500 ms).
- **Ambient effects** (neon grid + scanlines) from theme settings.

---

## 📦 Prebuilt Downloads

| OS      | Arch                     | Artifact (example)                 |
|---------|--------------------------|------------------------------------|
| Linux   | x86_64, aarch64          | `app-linux-x64.zip`, `app-linux-arm64.tar.gz` |
| macOS   | universal2 (x64+arm64)   | `app-macos-universal2.dmg`         |
| Windows | x86_64, arm64            | `app-windows-x64.zip`, `app-windows-arm64.zip` |

> If you’re building from source, see **Build from Source** below.

---

## 💾 Install

Download the archive for your platform from the table above and extract it.

### Linux

```bash
tar -xf app-linux-*.tar.gz   # or unzip app-linux-*.zip
./app                        # run the binary
```

### macOS

```bash
hdiutil attach app-macos-universal2.dmg
cp -R /Volumes/App/App.app /Applications
open /Applications/App.app
```

### Windows

```powershell
Expand-Archive app-windows-*.zip
Start-Process app\app.exe
```

---

## 🛠 Build from Source

### Prereqs
- **Rust**: pinned via `rust-toolchain.toml` (≥ 1.79.0)
- **GPU**: Vulkan (Linux), Metal (macOS), D3D12 (Windows)
- **Linux**: Vulkan loader (`libvulkan1`) + GPU drivers (Mesa/NVIDIA)
- **Windows**: Windows 10 **1809+** for ConPTY
- **macOS**: 12+ recommended

### Quick build
```bash
# in repo root
cargo build --release -p app
./target/release/app   # run (path varies per OS)
```

### Cross-platform targets (examples)

```bash
# Linux x64
cargo build --release -p app --target x86_64-unknown-linux-gnu

# Linux arm64
sudo apt-get update && sudo apt-get install -y gcc-aarch64-linux-gnu g++-aarch64-linux-gnu
cat >> ~/.cargo/config.toml <<'EOF2'
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
EOF2
cargo build --release -p app --target aarch64-unknown-linux-gnu

# macOS universal2
cargo build --release -p app --target x86_64-apple-darwin
cargo build --release -p app --target aarch64-apple-darwin
lipo -create \
  target/x86_64-apple-darwin/release/app \
  target/aarch64-apple-darwin/release/app \
  -output app-universal2

# Windows (PowerShell)
cargo build --release -p app --target x86_64-pc-windows-msvc
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

  1. `~/.config/edex-native/themes/*.toml` (Windows: `%APPDATA%\edex-native\themes\`)
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
* Windows: `%APPDATA%\edex-native\config.toml`

Example:

```toml
[appearance]
theme = "Tron Neon"
font_family = "JetBrains Mono"
font_size = 16

[render]
backend = "auto"         # auto|vulkan|metal|dx12
grid_spacing = 28
scanline_opacity = 0.06

[shell]
login = true
win_prefer_pwsh = true
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
uname -a            # (Windows: systeminfo)
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

* Matrix builds: Linux (x64/arm64), macOS (x64/arm64 → universal2), Windows (x64/arm64)
* Steps: `fmt`, `clippy -D warnings`, `test`, `build`
* Artifacts include binary + `assets/` (themes, shaders)

---

## 🐛 Troubleshooting

* **Windows Ctrl-C**: requires ConPTY (Win10 1809+). Prefer `pwsh.exe` if available.
* **Linux/Wayland**: ensure Vulkan loader (`libvulkan1`) and GPU driver installed.
* **No colors/Truecolor**: verify `TERM=xterm-256color`.
* **Huge output (e.g., `yes`)**: PTY → UI buffer is capped; rendering may rate-limit.

---

## 🙌 Credits

* **Fork maintainer**: **@ex3cutablefile** — project direction, migration plan, native UI, and theme system.
* Inspired by the original **eDEX-UI** concept. If you reuse original eDEX assets/themes, respect their **GPL-3.0** license.
* This Rust rewrite’s code is licensed as noted below.

---

## 📜 License

* **Rust code (this repository)**: choose a license appropriate for your goals (e.g., MIT or Apache-2.0).
* **Legacy eDEX assets** (if any are reused): **GPL-3.0**. Mixing GPL assets imposes GPL terms on the combined distribution. Consider shipping **original** themes/assets to keep the Rust code under a permissive license.

---

## 📬 Contributing

PRs and issues welcome!

* Run `cargo fmt`, `cargo clippy -D warnings`, `cargo test` before pushing.
* Add screenshots/gifs for UI PRs (themes/effects).
* Keep panels on a throttled update cadence; never block the terminal render path.

