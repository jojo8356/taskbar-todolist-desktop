# Story 4.1: Open Full Edit UI From Tray

Status: done

## Story

As a desktop user,
I want to open the full task editor from the tray,
so that quick capture stays lightweight while deeper edits remain available.

## Acceptance Criteria

1. Given app available in tray, when user chooses full UI action, then full edit window opens.
2. Given full edit window open, when user closes it, then app remains available in tray and tray panel behavior unchanged.
3. Given full edit UI rendered, when visual structure inspected, then it is utilitarian and secondary and does not replace quick add/delete tray behavior.

## Tasks / Subtasks

- [x] Add a Tauri command to open the full edit window from frontend code. (AC: 1)
- [x] Add a compact tray header action for opening the full editor. (AC: 1)
- [x] Create the full edit window lazily and hide it on close instead of exiting. (AC: 2)
- [x] Route the `full-edit` webview to the full editor UI while preserving the tray panel route. (AC: 3)
- [x] Run validation. (AC: 1-3)
  - [x] `cargo fmt --manifest-path src-tauri/Cargo.toml --check`
  - [x] `cargo test --manifest-path src-tauri/Cargo.toml`
  - [x] `pnpm build`

## Dev Notes

- The full edit window uses the existing Tauri webview bundle and is selected by window label.
- Closing the full edit window follows the same hide-on-close lifecycle as the tray panel.
- The tray panel keeps add/delete behavior in place; the full editor is a secondary path.

## Dev Agent Record

### Agent Model Used

GPT-5

### Debug Log References

- `cargo fmt --manifest-path src-tauri/Cargo.toml --check`
- `env PKG_CONFIG_PATH=/tmp/taskbar-todolist-native-pkgconfig PKG_CONFIG_SYSROOT_DIR=/tmp/taskbar-todolist-native-root LD_LIBRARY_PATH=/tmp/taskbar-todolist-native-root/usr/lib/x86_64-linux-gnu cargo test --manifest-path src-tauri/Cargo.toml`
- `pnpm build`

### Completion Notes List

- Added `show_full_edit` as a Tauri command and TypeScript app command wrapper.
- Added a full editor icon action to the tray panel header.
- Added a lazily created `full-edit` window with hide-on-close behavior.
- Rendered full edit UI only for the `full-edit` window label.

### File List

- `_bmad-output/implementation-artifacts/4-1-open-full-edit-ui-from-tray.md`
- `_bmad-output/implementation-artifacts/sprint-status.yaml`
- `src-tauri/src/app/windows.rs`
- `src-tauri/src/lib.rs`
- `src/main.ts`
- `src/state/app-commands.ts`
- `src/ui/tray-panel.ts`
- `src/ui/full-edit.ts`
- `src/styles.css`
