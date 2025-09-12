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
- Icons: builds reference `media/hackerui.icns` and `media/hackerui.ico`.
- Network widget: simplified and faster. No more `ifconfig`/`ipconfig`/`netstat` dumps.
  - Local IP: macOS `ipconfig getifaddr en0` (fallback en1), Linux `hostname -I | awk '{print $1}'`, Windows parses `ipconfig`.
  - Public IP: `curl -s ifconfig.me` (fallback `curl -s api.ipify.org`), cached for 60s.
  - Polling: Local every 5s, Public every 60s; respects adaptive throttling when unfocused/hidden.

Quick start
1) Requirements
- Node.js 22, npm 10+ (or pnpm/yarn if you prefer)
- macOS, Linux, or Windows

2) Install and run (development)
- Install deps: `npm install`
- Start: `npm start`
  - This launches Electron 12 with the app in fullscreen. Use F11 if windowed mode is allowed.

3) Build binaries
- macOS (x64 + arm64):
  - Prebuild native modules: `npm run prebuild-darwin`
  - Build: `npm run build-darwin`
  - Output: `dist/HackerUI-macOS-arm64.dmg`, `dist/HackerUI-macOS-x64.dmg`
- Linux (x64/ia32/arm/arm64):
  - Prebuild: `npm run prebuild-linux`
  - Build: `npm run build-linux`
  - Output: `dist/HackerUI-Linux-<arch>.AppImage`
- Windows (x64/ia32):
  - Prebuild: `npm run prebuild-windows`
  - Build: `npm run build-windows`
  - Output: `dist/HackerUI-Windows-<arch>.exe`

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
- Linux: `media/linuxIcons` directory
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

## Useful commands for the nerds

**IMPORTANT NOTE:** the following instructions are meant for running eDEX from the latest unoptimized, unreleased, development version. If you'd like to get stable software instead, refer to [these](#how-do-i-get-it) instructions.

#### Starting from source:
on *nix systems (You'll need the Xcode command line tools on macOS):
- clone the repository
- `npm run install-linux`
- `npm run start`

on Windows:
- start cmd or powershell **as administrator**
- clone the repository
- `npm run install-windows`
- `npm run start`

#### Building
Note: Due to native modules, you can only build targets for the host OS you are using.

- `npm install` (NOT `install-linux` or `install-windows`)
- `npm run build-linux` or `build-windows` or `build-darwin`

The script will minify the source code, recompile native dependencies and create distributable assets in the `dist` folder.

#### Getting the bleeding edge
If you're interested in running the latest in-development version but don't want to compile source code yourself, you can can get pre-built nightly binaries on [GitHub Actions](https://github.com/GitSquared/edex-ui/actions): click the latest commits, and download the artifacts bundle for your OS.

## Credits
eDEX-UI's source code was primarily written by me, [Squared](https://github.com/GitSquared). If you want to get in touch with me or find other projects I'm involved in, check out [my website](https://gaby.dev).

[PixelyIon](https://github.com/PixelyIon) helped me get started with Windows compatibility and offered some precious advice when I started to work on this project seriously.

[IceWolf](https://soundcloud.com/iamicewolf) composed the sound effects on v2.1.x and above. He makes really cool stuff, check out his music!

## Thanks
Of course, eDEX would never have existed if I hadn't stumbled upon the amazing work of [Seena](https://github.com/seenaburns) on [r/unixporn](https://reddit.com/r/unixporn).

This project uses a bunch of open-source libraries, frameworks and tools, see [the full dependency graph](https://github.com/GitSquared/edex-ui/network/dependencies).

I want to namely thank the developers behind [xterm.js](https://github.com/xtermjs/xterm.js), [systeminformation](https://github.com/sebhildebrandt/systeminformation) and [SmoothieCharts](https://github.com/joewalnes/smoothie).

Huge thanks to [Rob "Arscan" Scanlon](https://github.com/arscan) for making the fantastic [ENCOM Globe](https://github.com/arscan/encom-globe), also inspired by the TRON: Legacy movie, and distributing it freely. His work really puts the icing on the cake.

## Licensing

Licensed under the [GPLv3.0](https://github.com/GitSquared/edex-ui/blob/master/LICENSE).
