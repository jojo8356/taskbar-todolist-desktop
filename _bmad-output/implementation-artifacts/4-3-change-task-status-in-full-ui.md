# Story 4.3: Change Task Status in Full UI

Status: done

## Story

As a desktop user,
I want to change a task status in the full UI,
so that tasks can move between todo and done without adding heavier task management concepts.

## Acceptance Criteria

1. Given task selected, when user changes status, task status can be saved as `todo` or `done`.
2. Given status save succeeds, changed status persists after restart.
3. Given status changes, tray panel uses same shared task data source as full UI.
4. Scope inspected: no projects/tags/priorities/calendar/kanban/collaboration/heavy settings.

## Tasks / Subtasks

- [x] Add a status field constrained to `todo` and `done`. (AC: 1)
- [x] Save status changes through the persisted Tauri task update command. (AC: 1, 2)
- [x] Keep tray and full editor reads on the same list command/source. (AC: 3)
- [x] Keep scope limited to text and status editing only. (AC: 4)
- [x] Run validation. (AC: 1-4)
  - [x] `cargo fmt --manifest-path src-tauri/Cargo.toml --check`
  - [x] `cargo test --manifest-path src-tauri/Cargo.toml`
  - [x] `pnpm build`

## Dev Notes

- No schema change was required because Epic 3 already persisted `status`.
- Status changes use the existing `update_task` Rust command, so they persist in SQLite and survive restart.
- No advanced task management concepts were added.

## Dev Agent Record

### Agent Model Used

GPT-5

### Debug Log References

- `cargo fmt --manifest-path src-tauri/Cargo.toml --check`
- `env PKG_CONFIG_PATH=/tmp/taskbar-todolist-native-pkgconfig PKG_CONFIG_SYSROOT_DIR=/tmp/taskbar-todolist-native-root LD_LIBRARY_PATH=/tmp/taskbar-todolist-native-root/usr/lib/x86_64-linux-gnu cargo test --manifest-path src-tauri/Cargo.toml`
- `pnpm build`

### Completion Notes List

- Added a full editor status select limited to Todo and Done.
- Saved status and text together through the shared update command.
- Kept the full editor intentionally narrow: no projects, tags, priorities, calendar, kanban, collaboration, or heavy settings.

### File List

- `_bmad-output/implementation-artifacts/4-3-change-task-status-in-full-ui.md`
- `_bmad-output/implementation-artifacts/sprint-status.yaml`
- `src/ui/full-edit.ts`
- `src/styles.css`
