# Story 3.2: Rust Task Repository and Tauri Commands

Status: done

<!-- Note: Validation is optional. Run validate-create-story for quality check before dev-story. -->

## Story

As a desktop user,
I want task operations to be saved through the native app layer,
so that task behavior is reliable and consistent across tray and full UI.

## Acceptance Criteria

1. Given SQLx persistence is available, when the task repository is implemented, then it supports create, list active, update, and soft delete operations.
2. Given Tauri commands are registered, when the frontend invokes task operations, then `create_task`, `list_tasks`, `update_task`, and `delete_task` are available.
3. Given command DTOs cross the Rust/TypeScript boundary, when task data is returned to the frontend, then TypeScript receives camelCase fields and Rust/database internals remain snake_case.
4. Given a validation or storage error occurs, when a command fails, then the error maps to a stable machine-readable code and compact message.

## Tasks / Subtasks

- [x] Extend repository operations. (AC: 1)
  - [x] Replace smoke-only create/read with production repository methods: `create`, `list_active`, `update`, and `soft_delete`.
  - [x] Validate non-empty task text before insert/update and preserve existing text/status when optional update fields are absent.
  - [x] Keep active list queries filtered by `deleted_at IS NULL`.
  - [x] Add repository unit tests for create, list active, update text/status, not-found update/delete, and soft delete filtering.
- [x] Add command DTO and error boundary. (AC: 3, 4)
  - [x] Add `src-tauri/src/tasks/dto.rs` with camelCase serialized task DTOs and update payload types.
  - [x] Add `src-tauri/src/app/errors.rs` with stable codes: `TASK_TEXT_EMPTY`, `TASK_NOT_FOUND`, `TASK_STORAGE_ERROR`, `TASK_VALIDATION_ERROR`, `APP_INTERNAL_ERROR`.
  - [x] Ensure raw SQLx/debug errors are not exposed as command messages.
  - [x] Keep ISO 8601 UTC strings at the command boundary.
- [x] Register Tauri task commands. (AC: 2, 4)
  - [x] Add `src-tauri/src/tasks/commands.rs` with `create_task`, `list_tasks`, `update_task`, and `delete_task`.
  - [x] Register commands in `src-tauri/src/lib.rs`.
  - [x] Keep repository access through `AppState`; do not open SQLite from commands directly.
- [x] Add frontend command wrappers and types. (AC: 2, 3)
  - [x] Add `src/types/task.ts` with camelCase frontend task types and `TaskStatus`.
  - [x] Add `src/state/task-commands.ts` wrapping Tauri `invoke(...)`.
  - [x] Keep UI modules free of raw task `invoke(...)` calls.
- [x] Perform story validation. (AC: 1-4)
  - [x] Run `cargo fmt --check`.
  - [x] Run `cargo test --manifest-path src-tauri/Cargo.toml`.
  - [x] Run `pnpm build`.
  - [x] Attempt controlled `pnpm tauri dev` validation.

## Dev Notes

### Previous Story Intelligence

- Story 3.1 is complete: SQLite migration, SQLx migration runner, `TaskRepository`, `Task` model, app state initialization, local native dependency vendoring, and repository smoke test exist.
- Use `scripts/setup-native-deps.sh` if `/tmp/taskbar-todolist-native-root` or `/tmp/taskbar-todolist-native-pkgconfig` symlinks are missing before Cargo/Tauri validation.
- `cargo test --manifest-path src-tauri/Cargo.toml`, `pnpm build`, and controlled `pnpm tauri dev` passed after 3.1.

### Architecture Guardrails

- The only frontend persistence boundary is `TypeScript UI -> state/task-commands.ts -> Tauri invoke -> Rust commands.rs -> repository.rs -> SQLite`. [Source: _bmad-output/planning-artifacts/architecture.md#Architectural-Boundaries]
- Initial Tauri command names are `create_task`, `list_tasks`, `update_task`, and `delete_task`. [Source: _bmad-output/planning-artifacts/architecture.md#API--Communication-Patterns]
- Rust commands should return `Result<T, AppError>` and frontend wrappers should convert command results into UI-friendly types. [Source: _bmad-output/planning-artifacts/architecture.md#Error-Handling]
- Error format must include stable code and compact message; initial codes are `TASK_TEXT_EMPTY`, `TASK_NOT_FOUND`, `TASK_STORAGE_ERROR`, `TASK_VALIDATION_ERROR`, `APP_INTERNAL_ERROR`. [Source: _bmad-output/planning-artifacts/architecture.md#Error-Format]
- Rust/database fields may remain snake_case internally, but command DTOs exposed to TypeScript must serialize camelCase. [Source: _bmad-output/planning-artifacts/architecture.md#Data-Exchange-Format]
- Payloads must be minimal: `create_task { text }`, `update_task { id, text?, status? }`, `delete_task { id }`, `list_tasks` with no payload. [Source: _bmad-output/planning-artifacts/architecture.md#Command-Payloads]
- Active task queries must filter soft-deleted rows and user-facing delete must set `deleted_at`, not hard-delete. [Source: _bmad-output/planning-artifacts/architecture.md#Delete-Pattern]
- Any new task persistence function must be added to `repository.rs` and exposed through `commands.rs`; any frontend task operation must go through `state/task-commands.ts`. [Source: _bmad-output/planning-artifacts/architecture.md#Pattern-Enforcement]

### Project Structure Notes

- Expected Rust files: `src-tauri/src/app/errors.rs`, `src-tauri/src/tasks/commands.rs`, `src-tauri/src/tasks/dto.rs`, `src-tauri/src/tasks/repository.rs`, `src-tauri/src/tasks/model.rs`, `src-tauri/src/lib.rs`.
- Expected TypeScript files: `src/types/task.ts`, `src/state/task-commands.ts`.
- Do not add frontend SQLite, Node ORM, backend, cloud, account, mobile, or sync dependencies.
- Keep existing tray lifecycle commands and explicit quit behavior from Epic 1 intact.

### References

- [Source: _bmad-output/planning-artifacts/epics.md#Story-3-2-Rust-Task-Repository-and-Tauri-Commands]
- [Source: _bmad-output/planning-artifacts/architecture.md#API--Communication-Patterns]
- [Source: _bmad-output/planning-artifacts/architecture.md#Architectural-Boundaries]
- [Source: _bmad-output/planning-artifacts/architecture.md#Pattern-Enforcement]
- [Source: _bmad-output/implementation-artifacts/3-1-sqlx-sqlite-task-schema-and-migrations.md#Dev-Agent-Record]

## Dev Agent Record

### Agent Model Used

GPT-5 Codex

### Debug Log References

- `cargo fmt --manifest-path src-tauri/Cargo.toml --check` passed.
- `cargo test --manifest-path src-tauri/Cargo.toml` passed with 6 repository tests.
- `pnpm build` passed.
- `timeout 20s pnpm tauri dev` started Vite, compiled Cargo, and launched the app; process ended by the controlled timeout.

### Completion Notes List

- Repository now supports create, active list, update, and soft delete operations.
- Added stable application error mapping and command DTOs that serialize camelCase fields to TypeScript.
- Registered `create_task`, `list_tasks`, `update_task`, and `delete_task` Tauri commands.
- Added frontend `Task` types and `state/task-commands.ts` wrappers so UI code has a single command boundary.
- Added repository tests for create/read, text validation, active filtering, update, not-found paths, and tombstone preservation.

### Change Log

- 2026-05-22: Completed Story 3.2 implementation and validation.

### File List

- `_bmad-output/implementation-artifacts/3-2-rust-task-repository-and-tauri-commands.md`
- `_bmad-output/implementation-artifacts/sprint-status.yaml`
- `src-tauri/src/app/errors.rs`
- `src-tauri/src/app/mod.rs`
- `src-tauri/src/app/state.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/src/tasks/commands.rs`
- `src-tauri/src/tasks/dto.rs`
- `src-tauri/src/tasks/mod.rs`
- `src-tauri/src/tasks/model.rs`
- `src-tauri/src/tasks/repository.rs`
- `src/state/task-commands.ts`
- `src/types/task.ts`
