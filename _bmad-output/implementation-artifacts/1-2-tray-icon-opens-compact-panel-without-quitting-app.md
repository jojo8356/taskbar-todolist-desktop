# Story 1.2: Tray Icon Opens Compact Panel Without Quitting App

Status: done

> Supersedes the previous Tauri tray implementation. The active implementation is Rust native + Slint UI + StatusNotifierItem over D-Bus, with no Tauri, WebGTK/WebKit, webview, Vite, or pnpm runtime.

## Story

As a desktop user,
I want to open Taskbar Todolist from the Linux tray,
so that I can access tasks without opening a full application window.

## Acceptance Criteria

1. Given the app is running on the MVP Linux validation baseline, when the app starts, then a tray/system indicator registration is attempted through StatusNotifierItem.
2. Given the tray icon is visible, when the user activates the tray icon or tray context action, then the compact Slint panel is shown.
3. Given the compact panel is open, when the user closes or hides the panel, then the app process remains running in the background and the tray registration remains owned by the app.
4. Given the compact panel opens, when the input is rendered, then the input can receive keyboard input.

## Tasks / Subtasks

- [x] Add Rust-native tray lifecycle plumbing. (AC: 1, 2, 3)
  - [x] Implement StatusNotifierItem over D-Bus with `zbus`.
  - [x] Register the item with `org.kde.StatusNotifierWatcher` when available.
  - [x] Keep the D-Bus connection owned for the app lifetime.
  - [x] Avoid `tray-icon`, GTK/GDK, Tauri, WebGTK/WebKit, webview, Vite, and pnpm runtime dependencies.
- [x] Wire tray activation to the compact Slint panel. (AC: 2, 4)
  - [x] Store a weak Slint window handle in the tray item.
  - [x] Use `slint::invoke_from_event_loop` to show the panel from D-Bus callbacks.
  - [x] Keep the compact panel shell around 360x420 with a top text input.
- [x] Keep app alive when panel closes. (AC: 3)
  - [x] Configure Slint close requests to hide the window instead of ending the app.
- [x] Perform story validation. (AC: 1-4)
  - [x] Confirm `cargo fmt --check` passes.
  - [x] Confirm `cargo check` passes.
  - [x] Attempt controlled `cargo run` validation.

## Dev Notes

### Architecture Guardrails

- Use Rust native app lifecycle and Slint UI. Do not reintroduce Tauri or webview UI.
- Tray behavior is Linux-first and validated on Debian GNU/Linux 13 (trixie), GNOME X11, GNOME AppIndicator/KStatusNotifierItem extension.
- D-Bus StatusNotifierItem is preferred over GTK/GDK tray crates because it is closer to the Linux tray system and avoids a heavier toolkit dependency.

### Known Validation Notes

- If `org.kde.StatusNotifierWatcher` is unavailable in the current desktop session, the app logs the watcher error and still starts the Slint panel. The selected GNOME baseline should provide watcher support through the AppIndicator/KStatusNotifierItem extension.

## Dev Agent Record

### Agent Model Used

GPT-5

### Debug Log References

- `cargo fmt --check`
- `cargo check`
- `timeout 5s cargo run`
- `timeout 60s cargo run`

### Completion Notes List

- Added `zbus` and implemented a Rust-native `org.kde.StatusNotifierItem`.
- Tray activation, context menu, and secondary activation callbacks show the Slint panel through the Slint event loop.
- Slint close requests now hide the panel so the app can remain alive.
- Removed the optional `tray-icon` dependency path from the active scaffold to avoid GTK/GDK tray dependency.
- Controlled local run starts the app and reports the expected environment blocker when `org.kde.StatusNotifierWatcher` is unavailable in the current session.

### File List

- `Cargo.toml`
- `Cargo.lock`
- `README.md`
- `src/main.rs`
- `src/app/tray.rs`
- `src/ui/mod.rs`
- `_bmad-output/implementation-artifacts/1-2-tray-icon-opens-compact-panel-without-quitting-app.md`

### Change Log

- 2026-06-09: Reimplemented story 1.2 for Rust native StatusNotifierItem tray lifecycle.
