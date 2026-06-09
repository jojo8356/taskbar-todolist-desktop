# Taskbar Todolist Desktop

Linux-first Rust native desktop app for quick task capture from the system tray.

## Stack

- Rust native application shell
- Slint for the complete lightweight UI module
- Rust-native Linux StatusNotifierItem tray integration through D-Bus
- SQLite persistence through SQLx
- Debian GNU/Linux 13 (trixie), GNOME X11, GNOME AppIndicator/KStatusNotifierItem extension as the MVP validation baseline

The app intentionally does not use Tauri, WebGTK/WebKit, webviews, Vite, pnpm, or a browser-based UI runtime.

## Development

```bash
cargo run
```

Native Linux packages expected for the validation baseline:

```bash
sudo apt install build-essential pkg-config libssl-dev libdbus-1-dev libayatana-appindicator3-dev librsvg2-dev
```

No WebGTK/WebKit, GTK/GDK tray crate, webview, Tauri, Vite, or pnpm dependency is required for the active scaffold.

## Validation Baseline

- Distribution: Debian GNU/Linux 13 (trixie)
- Desktop/session: GNOME X11
- Tray support: GNOME AppIndicator/KStatusNotifierItem extension
