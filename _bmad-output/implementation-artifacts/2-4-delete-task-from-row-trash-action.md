# Story 2.4: Delete Task From Row Trash Action

Status: done

## Story

As a desktop user,
I want to delete a task from the trash icon on its row,
so that I can clean up my list without opening the full UI.

## Acceptance Criteria

1. Given an active task is visible in the tray panel, when the row is rendered, then a trash icon button appears on the right side of that row.
2. Given the user clicks the row trash action, when the delete operation succeeds, then the task is removed from the active list.
3. Given the user deletes a task from the tray, when the delete action is submitted, then deletion goes through the shared task command boundary and the UI updates the active list only after the command succeeds.
4. Given a trash button is rendered, when accessibility information is inspected, then it has an accessible label indicating that it deletes the task.
5. Given a task row is hovered or focused, when the trash action becomes visually accented, then the UI remains quiet and does not make deletion visually dominant.
6. Given the user deletes a task from the tray, when the action is complete, then the full edit UI was not required.

## Tasks / Subtasks

- [x] Render row trash icon button on the right. (AC: 1)
- [x] Submit delete through `deleteTask` command wrapper. (AC: 2, 3)
- [x] Remove task from active frontend cache only after command success. (AC: 2, 3)
- [x] Add accessible delete label. (AC: 4)
- [x] Keep danger hover quiet and token-aligned. (AC: 5)
- [x] Keep deletion entirely within tray panel. (AC: 6)
- [x] Run validation. (AC: 1-6)
  - [x] `cargo fmt --manifest-path src-tauri/Cargo.toml --check`
  - [x] `cargo test --manifest-path src-tauri/Cargo.toml`
  - [x] `pnpm build`

## Dev Notes

- Delete behavior was strengthened during Story 3.4 data safety work.
- User-facing delete is a soft delete in SQLite; active list filtering hides tombstones.
- Row button uses Lucide `Trash2` and an accessible label of `Delete task: {task.text}`.

## Dev Agent Record

### Agent Model Used

GPT-5

### Debug Log References

- `cargo fmt --manifest-path src-tauri/Cargo.toml --check`
- `env PKG_CONFIG_PATH=/tmp/taskbar-todolist-native-pkgconfig PKG_CONFIG_SYSROOT_DIR=/tmp/taskbar-todolist-native-root LD_LIBRARY_PATH=/tmp/taskbar-todolist-native-root/usr/lib/x86_64-linux-gnu cargo test --manifest-path src-tauri/Cargo.toml`
- `pnpm build`

### Completion Notes List

- Tray row delete is wired through the persisted command boundary.
- Frontend state updates only after delete success.
- Trash action stays quiet until hover and uses the documented danger token.
- Deleting from the tray does not require opening the full edit UI.

### File List

- `_bmad-output/implementation-artifacts/2-4-delete-task-from-row-trash-action.md`
- `_bmad-output/implementation-artifacts/sprint-status.yaml`
- `src/ui/task-row.ts`
- `src/ui/task-list.ts`
- `src/ui/tray-panel.ts`
- `src/state/task-commands.ts`
- `src/state/task-store.ts`
- `src/styles.css`
