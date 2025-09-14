<p align="center">
  <br>
  <img alt="HackerUI" src="media/logo.png" width="200">
  <br><br>
  <strong>HackerUI — a cyber-console environment by 3xecutable File</strong>
  <br>
  <em>Modded fork of the archived eDEX‑UI project.</em>
  <br><br>
</p>

HackerUI is a fullscreen, cross‑platform terminal emulator and system monitor that looks and feels like a sci‑fi computer interface. It’s a modded version of eDEX‑UI with refreshed branding, quality‑of‑life fixes, and an optimized network widget.

Highlights
- Modded fork of eDEX‑UI with HackerUI branding and boot sequence.
- Cleaned up “Network” widget that shows just Local and Public IP.
- Dual‑arch macOS builds with DMG volume label “HackerUI”.
- Node 22 + Electron 12 + node‑pty ^1.0.0 compatible.
- Same immersive UI: tabs, on‑screen keyboard, themes, and system stats.

What’s different from eDEX‑UI
- Branding: App name, product name, appId, window title, and boot text now say “HackerUI — by 3xecutable File”.
- DMG artifacts: `HackerUI-macOS-arm64.dmg` and `HackerUI-macOS-x64.dmg` with volume label “HackerUI”.
- Icons: builds reference `media/hackerui.icns` (macOS), `media/hackerui.ico` (Windows), `media/hackerui.png` (Linux).
- Network widget: simplified and faster. No more `ifconfig`/`ipconfig`/`netstat` dumps.
  - Local IP: macOS `ipconfig getifaddr en0` (fallback en1), Linux `hostname -I | awk '{print $1}'`, Windows parses `ipconfig`.
  - Public IP: `curl -s ifconfig.me` (fallback `curl -s api.ipify.org`), cached for 60s.
  - Polling: Local every 5s, Public every 60s; respects adaptive throttling when unfocused/hidden.

Quick start
1) Clone and enter
- `git clone https://github.com/3xecutable/terminal-ui.git`
- `cd terminal-ui`

2) Requirements
- Node.js 22, npm 10+
- macOS, Linux, or Windows

3) Install and run (development)
- Install deps: `npm install`
- Start: `npm start`
  - Launches Electron 12 in fullscreen (unless settings allow windowed mode).

4) Build binaries

- Ensure dependencies are installed (`npm install`) before running these commands.
=======

- Ensure dependencies are installed (`npm install`) before running these commands.
=======

- Ensure dependencies are installed (`npm install`) before running these commands.
=======



- macOS (x64 + arm64):
  - Prebuild native modules: `npm run prebuild-darwin`
  - Build: `npm run build-darwin`
  - Output: `dist/HackerUI-macOS-arm64.dmg`, `dist/HackerUI-macOS-x64.dmg`
- Linux (x64/arm64):
  - Prebuild: `npm run prebuild-linux`
  - Build: `npm run build-linux`
  - Output: `dist/HackerUI-Linux-<arch>.AppImage`
- Windows (x64):
  - Prebuild: `npm run prebuild-windows`
  - Build: `npm run build-windows`
  - Output: `dist/HackerUI-Windows-x64.exe`

5) Pin HackerUI to the macOS Dock
- Launch the app once, then right-click its Dock icon and choose “Options → Keep in Dock”, or drag `HackerUI.app` from `/Applications` onto the Dock.

Features
- Terminal emulator with tabs, colors, mouse, and curses support (xterm.js based).
- Real‑time system stats (CPU, RAM, processes) and network info.
- Touch‑friendly on‑screen keyboard and sci‑fi themed UI.
- Filesystem viewer that follows the terminal’s CWD on macOS/Linux.
- Theming support (custom colors, fonts, keyboards, and CSS injects).

Network widget details
- Local IP logic
  - macOS: `ipconfig getifaddr en0` then `en1` as fallback.
  - Linux: `hostname -I | awk '{print $1}'`.
  - Windows: parses `ipconfig` for “IPv4 Address”.
- Public IP logic
  - `curl -s ifconfig.me`, fallback `curl -s api.ipify.org`.
  - Cached for 60 seconds to avoid spamming requests.
- Refresh intervals
  - Local IP every 5s, Public IP every 60s; increased when app is unfocused/hidden if adaptive throttling is enabled.

Configuration
- HackerUI stores its settings under Electron’s userData path for your OS.
- Useful settings include:
  - `shell`, `shellArgs`, `cwd`, `theme`, `keyboard`, `termFontSize`, `termScrollback`.
  - `gpuAcceleration`, `adaptiveThrottling`, `statsRefreshMs`.
  - `pingAddr`, `port`, `nointro`, `nocursor`, `allowWindowed`, `keepGeometry`.

Assets and icons
- macOS icon: `media/hackerui.icns`
- Windows icon: `media/hackerui.ico`
- Linux icon: `media/hackerui.png`
- Replace these files with your own artwork to customize the look. The repo currently includes placeholders derived from the original icons.

Keyboard shortcuts
- App shortcuts are configurable in `shortcuts.json` (auto‑generated in userData on first run).
- Common actions (examples):
  - `Ctrl+X` with numbers 1–5 for tab switching
  - `Ctrl+Shift+P` toggle keyboard password mode
  - `Ctrl+Shift+I` open devtools (if enabled)

Troubleshooting
- If native modules fail to load, ensure you ran the appropriate `prebuild-*` script for your platform. These scripts rebuild node‑pty for Electron 12.
- On Linux, mark AppImage executable: `chmod +x HackerUI-Linux-*.AppImage`.
- If the UI feels sluggish when unfocused, that’s expected with adaptive throttling enabled; disable it in settings to force fast refresh.

Screenshots
![Default](media/screenshot_default.png)
![Blade](media/screenshot_blade.png)
![Disrupted](media/screenshot_disrupted.png)
![Horizon](media/screenshot_horizon.png)

Credits
- Original project: eDEX‑UI by Gabriel “Squared” Saillard (GitSquared).
- HackerUI: rebrand and modifications by 3xecutable File.
- License: GPL‑3.0 (see LICENSE). Please respect the license when redistributing.

Disclaimer
- HackerUI is an independent, modded fork of the archived eDEX‑UI project. It is not affiliated with or endorsed by the original author.

---

## 🧰 Native (Rust) Workspace

An experimental native rewrite lives under `native/` with a wgpu UI and a PTY daemon.

- PTY daemon (`native/ptyd`) speaks NDJSON:
  - Input frame: `{"t":"i","data":"<base64-bytes>"}`
  - Resize: `{"t":"r","cols":120,"rows":40}`
  - Signal: `{"t":"s","sig":"INT"}`
  - Output: `{"t":"o","data":"<base64-bytes>","seq":N}`
  - Exit: `{"t":"x","code":0}` (includes string `signal` when terminated by one, e.g., `{"t":"x","code":1,"signal":"Terminated"}`)
- Native app (`native/app`) renders with `wgpu`, feeds PTY → emulator → GPU.

### Native CI (GitHub Actions)

- Matrix builds: Linux (x64) and macOS (x64)
- Steps: `fmt`, `clippy -D warnings`, `test`, `build`
- Artifacts include binary + `assets/` (themes, shaders)

### Native Development

- Workspace lives under `native/`
- Run checks before submitting PRs:
  ```bash
  cargo fmt --all --manifest-path native/Cargo.toml -- --check
  cargo clippy --workspace --manifest-path native/Cargo.toml -- -D warnings
  cargo test --workspace --manifest-path native/Cargo.toml
  ```

### Keybinds
| Action               | Shortcut                |
|----------------------|-------------------------|
| Theme switcher       | `Ctrl/Cmd + Shift + T`  |
| Copy / Paste         | Standard OS shortcuts   |

### Native License

- Rust code in `native/`: MIT OR Apache-2.0
- Legacy eDEX assets (if reused): GPL-3.0

