# Story 3.3: Persist Tray Tasks Across App Restart

Status: done

<!-- Note: Validation is optional. Run validate-create-story for quality check before dev-story. -->

## Story

As a desktop user,
I want tasks added from the tray to be available after restarting the app,
so that I can trust the app as local storage.

## Acceptance Criteria

1. Given the user creates a task from the tray, when the command succeeds, then the task is written to SQLite and the UI updates only after command success.
2. Given tasks exist in SQLite, when the app starts, then active tasks load from local storage.
3. Given the app is closed and reopened, when the tray panel opens, then previously active tasks are visible.
4. Given task loading is measured with 500 locally stored tasks, when active tasks are loaded, then local list loading completes in under 500 ms on the development validation machine.

## Tasks / Subtasks

- [x] Connect tray input to persistent create. (AC: 1)
  - [x] Add Enter handling in `task-input.ts`.
  - [x] Call `createTask` through `state/task-commands.ts`.
  - [x] Update UI only after the command resolves successfully.
  - [x] Preserve input focus and clear text after successful add.
- [x] Load active tasks into tray UI. (AC: 2, 3)
  - [x] Add a small frontend task store/cache for active tasks.
  - [x] Add task list/row rendering modules for active tasks.
  - [x] Call `listTasks` on initial tray render.
  - [x] Reload tasks when the tray panel receives `tray-panel-opened`.
- [x] Add compact status handling. (AC: 1, 2)
  - [x] Show loading, success, empty, and error states without exposing raw technical errors.
  - [x] Keep the tray panel compact and avoid unrelated concepts.
- [x] Add performance validation. (AC: 4)
  - [x] Add or run repository-level active-list timing coverage for 500 local tasks.
- [x] Perform story validation. (AC: 1-4)
  - [x] Run `cargo fmt --check`.
  - [x] Run `cargo test --manifest-path src-tauri/Cargo.toml`.
  - [x] Run `pnpm build`.
  - [x] Attempt controlled `pnpm tauri dev` validation.

## Dev Notes

### Previous Story Intelligence

- Story 3.2 completed `create_task`, `list_tasks`, `update_task`, and `delete_task`; frontend wrappers exist in `src/state/task-commands.ts`.
- Task DTOs exposed to TypeScript use camelCase fields in `src/types/task.ts`.
- Repository active list already filters `deleted_at IS NULL`.

### Architecture Guardrails

- UI modules must call `state/task-commands.ts`, not raw `invoke(...)`. [Source: _bmad-output/planning-artifacts/architecture.md#Architectural-Boundaries]
- Create flow must call `createTask(text)`, then update the store/UI from the command result. [Source: _bmad-output/planning-artifacts/architecture.md#Data-Flow]
- Load flow must call `listTasks()`, then replace the frontend active task cache. [Source: _bmad-output/planning-artifacts/architecture.md#Data-Flow]
- SQLite is the source of truth; frontend state is only a rendering cache. [Source: _bmad-output/planning-artifacts/architecture.md#State-Management-Patterns]
- Local list loading must meet the 500-task / 500 ms requirement. [Source: _bmad-output/planning-artifacts/epics.md#Story-3-3-Persist-Tray-Tasks-Across-App-Restart]

### Project Structure Notes

- Expected frontend files: `src/state/task-store.ts`, `src/ui/task-list.ts`, `src/ui/task-row.ts`, `src/ui/task-input.ts`, `src/ui/tray-panel.ts`, `src/styles.css`.
- Expected Rust test changes may stay in `src-tauri/src/tasks/repository.rs`.
- Do not introduce backend, cloud, account, mobile, sync, or frontend SQLite dependencies.

### References

- [Source: _bmad-output/planning-artifacts/epics.md#Story-3-3-Persist-Tray-Tasks-Across-App-Restart]
- [Source: _bmad-output/planning-artifacts/architecture.md#Data-Flow]
- [Source: _bmad-output/planning-artifacts/architecture.md#State-Management-Patterns]
- [Source: _bmad-output/implementation-artifacts/3-2-rust-task-repository-and-tauri-commands.md#Dev-Agent-Record]

## Dev Agent Record

### Agent Model Used

GPT-5 Codex

### Debug Log References

- `cargo fmt --manifest-path src-tauri/Cargo.toml --check` passed.
- `cargo test --manifest-path src-tauri/Cargo.toml` passed with 7 repository tests, including 500-task list timing.
- `pnpm build` passed.
- `timeout 20s pnpm tauri dev` started Vite, compiled Cargo, and launched the app; process ended by the controlled timeout.

### Completion Notes List

- Tray input now creates tasks through the Tauri command wrapper on Enter.
- Tray UI updates only after `createTask` resolves successfully.
- Active tasks load on initial render and when the tray-open event fires.
- Added small frontend task store/cache plus task list and row renderers.
- Added compact loading/success/error status messages.
- Added 500-active-task repository timing coverage.

### Change Log

- 2026-05-22: Completed Story 3.3 implementation and validation.

### File List

- `_bmad-output/implementation-artifacts/3-3-persist-tray-tasks-across-app-restart.md`
- `_bmad-output/implementation-artifacts/sprint-status.yaml`
- `src-tauri/src/tasks/repository.rs`
- `src/state/task-store.ts`
- `src/ui/task-input.ts`
- `src/ui/task-list.ts`
- `src/ui/task-row.ts`
- `src/ui/tray-panel.ts`
- `src/styles.css`
