# Story 2.3: Active Task List Rendering

Status: done

## Story

As a desktop user,
I want to see active tasks directly below the input,
so that I can quickly review what is currently pending.

## Acceptance Criteria

1. Given active tasks exist, when the tray panel opens, then active tasks render directly below the input without navigation.
2. Given active tasks are rendered, when the tray panel loads task data, then it reads active tasks through the shared task store backed by Tauri commands.
3. Given a task row is rendered, when the task text is long, then the row remains stable and readable within the compact panel.
4. Given the task list is rendered, when the list is inspected, then each task row uses stable row height around 36px and task text appears on the left.
5. Given no active tasks exist, when the tray panel opens, then the empty state appears below the input and adding remains immediately available.

## Tasks / Subtasks

- [x] Render active task region directly below input/status area. (AC: 1)
- [x] Load tasks through `listTasks` and shared `TaskStore`. (AC: 2)
- [x] Keep long task text stable with ellipsis in compact rows. (AC: 3)
- [x] Use stable row height around 36px with text left and controls right. (AC: 4)
- [x] Render minimal empty state when no active tasks exist. (AC: 5)
- [x] Run validation. (AC: 1-5)
  - [x] `cargo fmt --manifest-path src-tauri/Cargo.toml --check`
  - [x] `cargo test --manifest-path src-tauri/Cargo.toml`
  - [x] `pnpm build`

## Dev Notes

- Implemented by `src/ui/task-list.ts`, `src/ui/task-row.ts`, `src/ui/tray-panel.ts`, and `src/state/task-store.ts`.
- `TaskRepository::list_active` filters soft-deleted tasks at the Rust repository layer.
- Long text remains single-line with `text-overflow: ellipsis`.

## Dev Agent Record

### Agent Model Used

GPT-5

### Debug Log References

- `cargo fmt --manifest-path src-tauri/Cargo.toml --check`
- `env PKG_CONFIG_PATH=/tmp/taskbar-todolist-native-pkgconfig PKG_CONFIG_SYSROOT_DIR=/tmp/taskbar-todolist-native-root LD_LIBRARY_PATH=/tmp/taskbar-todolist-native-root/usr/lib/x86_64-linux-gnu cargo test --manifest-path src-tauri/Cargo.toml`
- `pnpm build`

### Completion Notes List

- Active tasks render in the compact tray panel without navigation.
- Task loading uses the persisted shared command/store boundary.
- Empty state remains minimal: `No active tasks`.
- Row layout is stable around 36px and keeps task text on the left.

### File List

- `_bmad-output/implementation-artifacts/2-3-active-task-list-rendering.md`
- `_bmad-output/implementation-artifacts/sprint-status.yaml`
- `src/ui/task-list.ts`
- `src/ui/task-row.ts`
- `src/ui/tray-panel.ts`
- `src/state/task-store.ts`
- `src/styles.css`
