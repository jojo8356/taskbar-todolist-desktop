# Story 1.3: App Tray Controls and Explicit Quit

Status: done

> Supersedes the previous Tauri/TypeScript implementation. The active implementation is Rust native + Slint UI + StatusNotifierItem D-Bus tray lifecycle.

## Story

As a desktop user,
I want minimal tray controls and an explicit quit action,
so that I can understand and control the app lifecycle.

## Acceptance Criteria

1. Given the app is running in the tray, when the user opens available controls, then the app exposes minimal control actions needed for Phase 1.
2. Given the app is running in the background, when the user chooses explicit quit, then the app exits completely.
3. Given the app is running normally, when the app state is displayed, then the user can identify the general local desktop state from compact status UI.

## Tasks / Subtasks

- [x] Add minimal lifecycle controls. (AC: 1, 2)
  - [x] Add compact panel controls for hiding the panel and explicit quit.
  - [x] Keep lifecycle behavior in Rust/Slint, not TypeScript or webview code.
  - [x] Keep StatusNotifierItem activation as the Linux tray path for showing the panel.
- [x] Add explicit quit lifecycle behavior. (AC: 2)
  - [x] Add Rust `quit_app` lifecycle function.
  - [x] Wire panel quit affordance to `slint::quit_event_loop`.
  - [x] Keep close requests as hide-window behavior, separate from explicit quit.
- [x] Add compact app state visibility. (AC: 3)
  - [x] Add minimal status text identifying local desktop/tray process state.
  - [x] Do not add sync mode selection, settings screens, persistence expansion, backend, mobile, or task CRUD in this story.
- [x] Perform story validation. (AC: 1-3)
  - [x] Confirm `cargo fmt --check` passes.
  - [x] Confirm `cargo check` passes.
  - [x] Attempt controlled `cargo run` validation.

## Dev Notes

### Architecture Guardrails

- Use Rust native app lifecycle and Slint UI. Do not reintroduce Tauri or webview UI.
- Explicit quit is intentionally separate from close/hide; close hides the panel, quit exits the Slint event loop.
- Current session lacks `org.kde.StatusNotifierWatcher`; this is an environment blocker for visual tray validation, not a compile/runtime failure of the Rust native app.

## Dev Agent Record

### Agent Model Used

GPT-5

### Debug Log References

- `cargo fmt --check`
- `cargo check`
- `timeout 5s cargo run`

### Completion Notes List

- Added Slint `Hide` and `Quit` controls to the compact panel.
- Added `app::windows::quit_app` using `slint::quit_event_loop`.
- Preserved close-request behavior as hide-window via `CloseRequestResponse::HideWindow`.
- Updated compact status text to identify local desktop/tray process state.
- Controlled run remains alive until timeout and logs the expected missing StatusNotifierWatcher blocker in this session.

### File List

- `src/app/windows.rs`
- `src/ui/mod.rs`
- `_bmad-output/implementation-artifacts/1-3-app-tray-controls-and-explicit-quit.md`

### Change Log

- 2026-06-09: Reimplemented story 1.3 for Rust native Slint lifecycle controls and explicit quit.
