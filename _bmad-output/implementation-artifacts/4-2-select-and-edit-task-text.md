# Story 4.2: Select and Edit Task Text

Status: done

## Story

As a desktop user,
I want to select a task and edit its text in the full UI,
so that I can fix or refine tasks after quick capture.

## Acceptance Criteria

1. Given active tasks exist, when full edit UI opens, user can select a task.
2. Given task selected, when user edits text and saves, task text updated through shared Tauri command boundary.
3. Given edit succeeds, when tray panel next renders, updated text visible there too.
4. Given edit command fails, compact user-facing status appears and task store does not apply unpersisted edit.

## Tasks / Subtasks

- [x] Render active tasks as selectable rows in the full editor. (AC: 1)
- [x] Add a task detail form with editable text. (AC: 2)
- [x] Save edited text through `updateTask(...)` and the Rust command boundary. (AC: 2)
- [x] Update the shared frontend store only after the command succeeds. (AC: 3, 4)
- [x] Show compact save/load failure statuses without applying failed edits. (AC: 4)
- [x] Run validation. (AC: 1-4)
  - [x] `cargo fmt --manifest-path src-tauri/Cargo.toml --check`
  - [x] `cargo test --manifest-path src-tauri/Cargo.toml`
  - [x] `pnpm build`

## Dev Notes

- The full editor uses the same `listTasks` and `updateTask` frontend command boundary as the tray-backed task data.
- The tray panel reloads tasks from the same persisted source when opened, so successful edits become visible there on next render.
- Empty text is rejected client-side without calling the update command.

## Dev Agent Record

### Agent Model Used

GPT-5

### Debug Log References

- `cargo fmt --manifest-path src-tauri/Cargo.toml --check`
- `env PKG_CONFIG_PATH=/tmp/taskbar-todolist-native-pkgconfig PKG_CONFIG_SYSROOT_DIR=/tmp/taskbar-todolist-native-root LD_LIBRARY_PATH=/tmp/taskbar-todolist-native-root/usr/lib/x86_64-linux-gnu cargo test --manifest-path src-tauri/Cargo.toml`
- `pnpm build`

### Completion Notes List

- Added selectable full editor rows with selected state.
- Added task text editing and save flow.
- Added `TaskStore.update(...)` for success-only local replacement.
- Preserved compact status messaging for load/save failures.

### File List

- `_bmad-output/implementation-artifacts/4-2-select-and-edit-task-text.md`
- `_bmad-output/implementation-artifacts/sprint-status.yaml`
- `src/state/task-store.ts`
- `src/ui/full-edit.ts`
- `src/styles.css`
