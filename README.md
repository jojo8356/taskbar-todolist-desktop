# Taskbar Todolist Desktop

Linux-first Rust native desktop app for quick task capture from the system tray.

## Stack

- Rust native application shell
- Slint for the complete lightweight UI module
- Optional lightweight Rust tray integration through the `tray` feature
- SQLite persistence through SQLx
- Debian GNU/Linux 13 (trixie), GNOME X11, GNOME AppIndicator/KStatusNotifierItem extension as the MVP validation baseline

The app intentionally does not use Tauri, WebGTK/WebKit, webviews, Vite, pnpm, or a browser-based UI runtime.

## Development

```bash
cargo run
```

Tray integration is prepared as an optional feature so the base scaffold stays light and does not require GTK/GDK packages before tray validation:

```bash
cargo run --features tray
```

Native Linux packages expected for the validation baseline:

```bash
sudo apt install build-essential pkg-config libssl-dev libdbus-1-dev libayatana-appindicator3-dev librsvg2-dev
```

When validating the optional tray feature, install the additional GTK/GDK development package required by `tray-icon` on Debian-family systems:

```bash
sudo apt install libgtk-3-dev
```

## Validation Baseline

- Distribution: Debian GNU/Linux 13 (trixie)
- Desktop/session: GNOME X11
- Tray support: GNOME AppIndicator/KStatusNotifierItem extension
