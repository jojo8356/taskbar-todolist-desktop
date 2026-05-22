# Story 3.4: Soft Delete and Local Data Safety

Status: done

<!-- Note: Validation is optional. Run validate-create-story for quality check before dev-story. -->

## Story

As a desktop user,
I want deleted tasks and failed operations handled safely,
so that local data is not lost or corrupted.

## Acceptance Criteria

1. Given the user deletes a task, when the delete command succeeds, then the task receives a `deleted_at` value and active task queries hide the task.
2. Given a deleted task exists, when raw storage is inspected for future sync readiness, then the deleted task record remains available as a tombstone.
3. Given a task mutation fails, when the command returns an error, then the frontend task store does not apply the failed mutation as if it succeeded.
4. Given simulated failure conditions such as invalid payload or interrupted operation, when local task state is compared before and after the test, then no local task data is deleted or corrupted.
5. Given the app idles without active sync, when CPU and memory are measured over 5 minutes, then the app remains under 150 MB RSS and under 2% average CPU on the validation environment.

## Tasks / Subtasks

- [x] Wire tray delete through command success. (AC: 1, 3)
  - [x] Add delete callback to task row/list rendering.
  - [x] Call `deleteTask` through `state/task-commands.ts`.
  - [x] Remove the row from the frontend active cache only after command success.
  - [x] Show compact failure status and preserve the row if command fails.
- [x] Strengthen local data safety tests. (AC: 1, 2, 4)
  - [x] Ensure repository soft delete keeps tombstone rows and hides them from active list.
  - [x] Ensure failed delete/update paths do not mutate existing active task data.
  - [x] Ensure invalid task text failures do not create records.
- [x] Perform runtime/resource validation. (AC: 5)
  - [x] Attempt controlled `pnpm tauri dev` validation.
  - [x] Run release idle CPU/RSS validation and record the result.
- [x] Perform story validation. (AC: 1-5)
  - [x] Run `cargo fmt --check`.
  - [x] Run `cargo test --manifest-path src-tauri/Cargo.toml`.
  - [x] Run `pnpm build`.

## Dev Notes

### Previous Story Intelligence

- Story 3.2 added `delete_task` and repository `soft_delete`.
- Story 3.3 added a frontend task store/cache, task list rendering, and tray task loading.
- Existing repository tests already cover tombstone preservation and active-list filtering; extend them for failed mutation safety.

### Architecture Guardrails

- User-facing delete must set `deleted_at`, not hard-delete rows. [Source: _bmad-output/planning-artifacts/architecture.md#Delete-Pattern]
- Frontend state must mutate only after successful command completion. [Source: _bmad-output/planning-artifacts/architecture.md#State-Management-Patterns]
- Raw storage errors must not be exposed in user-facing UI messages. [Source: _bmad-output/planning-artifacts/architecture.md#Error-Format]
- Keep Phase 1 free of sync, backend, account, cloud, or mobile dependencies. [Source: _bmad-output/planning-artifacts/architecture.md#Pattern-Enforcement]

### Project Structure Notes

- Expected frontend files: `src/state/task-store.ts`, `src/ui/task-list.ts`, `src/ui/task-row.ts`, `src/ui/tray-panel.ts`, `src/styles.css`.
- Expected Rust test changes may stay in `src-tauri/src/tasks/repository.rs`.

### References

- [Source: _bmad-output/planning-artifacts/epics.md#Story-3-4-Soft-Delete-and-Local-Data-Safety]
- [Source: _bmad-output/planning-artifacts/architecture.md#Delete-Pattern]
- [Source: _bmad-output/planning-artifacts/architecture.md#State-Management-Patterns]
- [Source: _bmad-output/implementation-artifacts/3-3-persist-tray-tasks-across-app-restart.md#Dev-Agent-Record]

## Dev Agent Record

### Agent Model Used

GPT-5

### Debug Log References

- `cargo fmt --manifest-path src-tauri/Cargo.toml --check`
- `cargo test --manifest-path src-tauri/Cargo.toml`
- `pnpm build`
- `timeout 20s pnpm tauri dev`
- `cargo tree --manifest-path src-tauri/Cargo.toml -i libappindicator`
- `cargo build --release --manifest-path src-tauri/Cargo.toml`
- `timeout 330s src-tauri/target/release/taskbar-todolist-desktop`
- `/tmp/taskbar-todolist-bench/measure.sh project-lazy-window src-tauri/target/release/taskbar-todolist-desktop`

### Completion Notes List

- Wired row-level delete to the persisted `delete_task` command and only removes tasks from the active frontend cache after command success.
- Added compact delete failure status handling so failed deletes preserve the visible row.
- Added repository safety tests for failed updates/deletes and invalid creates.
- Verified the direct tray migration removes the active `libappindicator` dependency and the runtime no longer emits the `libayatana-appindicator` deprecation warning.
- Implemented lazy Tauri WebView creation so idle startup does not load the tray panel WebKit runtime before first tray activation.
- Release idle validation now passes AC 5: final 5-minute sample was 72,056 KiB RSS, 39,250 KiB PSS, 18,588 KiB private dirty, and 0.0% CPU.

### File List

- `_bmad-output/implementation-artifacts/1-2-tray-icon-opens-compact-panel-without-quitting-app.md`
- `_bmad-output/implementation-artifacts/3-4-soft-delete-and-local-data-safety.md`
- `_bmad-output/implementation-artifacts/sprint-status.yaml`
- `src-tauri/Cargo.toml`
- `src-tauri/Cargo.lock`
- `src-tauri/src/app/mod.rs`
- `src-tauri/src/app/tray.rs`
- `src-tauri/src/app/windows.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/tauri.conf.json`
- `src-tauri/src/tasks/repository.rs`
- `src/state/task-store.ts`
- `src/ui/task-list.ts`
- `src/ui/task-row.ts`
- `src/ui/tray-panel.ts`
