# Story 2.2: Add Task From Top Input With Enter

Status: done

## Story

As a desktop user,
I want to type a task in the top input and press Enter,
so that I can capture a task without breaking my workflow.

## Acceptance Criteria

1. Given the tray panel opens, when the panel is visible, then the task input is fixed at the top and focused automatically.
2. Given the input contains non-empty text, when the user presses Enter, then a new simple task is created and appears in the active task list.
3. Given the task command boundary from Epic 3 is available, when the user adds a task from the tray, then the UI creates the task through `task-commands.ts` and does not create a frontend-only task record that bypasses persistence.
4. Given a task is added successfully, when the add operation completes, then the input is cleared and focus remains in the input for another task.
5. Given the input is empty, when the user presses Enter, then no task is created and no heavy error message is displayed.
6. Given the user adds a task from the tray, when measured from tray access to visible task, then the flow can be completed in under 5 seconds by the user.

## Tasks / Subtasks

- [x] Keep task input at the top of the compact panel. (AC: 1)
- [x] Autofocus/select input on panel open. (AC: 1)
- [x] Create tasks through `createTask` from `state/task-commands.ts`. (AC: 2, 3)
- [x] Add created task to shared frontend task store and re-render active list. (AC: 2)
- [x] Clear and refocus input after successful add. (AC: 4)
- [x] Ignore empty Enter without heavy error UI. (AC: 5)
- [x] Run validation. (AC: 1-6)
  - [x] `cargo fmt --manifest-path src-tauri/Cargo.toml --check`
  - [x] `cargo test --manifest-path src-tauri/Cargo.toml`
  - [x] `pnpm build`

## Dev Notes

- Implemented in `src/ui/task-input.ts` and composed in `src/ui/tray-panel.ts`.
- Persistence path is `src/ui/task-input.ts` -> `src/ui/tray-panel.ts` -> `src/state/task-commands.ts` -> Tauri command -> Rust repository -> SQLite.
- Empty input handling remains silent and lightweight.

## Dev Agent Record

### Agent Model Used

GPT-5

### Debug Log References

- `cargo fmt --manifest-path src-tauri/Cargo.toml --check`
- `env PKG_CONFIG_PATH=/tmp/taskbar-todolist-native-pkgconfig PKG_CONFIG_SYSROOT_DIR=/tmp/taskbar-todolist-native-root LD_LIBRARY_PATH=/tmp/taskbar-todolist-native-root/usr/lib/x86_64-linux-gnu cargo test --manifest-path src-tauri/Cargo.toml`
- `pnpm build`

### Completion Notes List

- Enter-to-add is wired through the persisted command boundary.
- Successful adds clear and refocus the input.
- Empty Enter does nothing without showing a heavy error.
- The created task appears immediately through the local task store update.

### File List

- `_bmad-output/implementation-artifacts/2-2-add-task-from-top-input-with-enter.md`
- `_bmad-output/implementation-artifacts/sprint-status.yaml`
- `src/ui/task-input.ts`
- `src/ui/tray-panel.ts`
- `src/state/task-commands.ts`
- `src/state/task-store.ts`
