# Story 3.1: SQLx SQLite Task Schema and Migrations

Status: in-progress

<!-- Note: Validation is optional. Run validate-create-story for quality check before dev-story. -->

## Story

As a desktop user,
I want tasks stored in a local SQLite database,
so that my tasks survive app restarts without any cloud dependency.

## Acceptance Criteria

1. Given the app starts, when the database is initialized, then SQLx migrations run before task commands are available.
2. Given the task schema is created, when the migration is inspected, then the `tasks` table includes `id`, `text`, `status`, `created_at`, `updated_at`, and nullable `deleted_at`.
3. Given the architecture requires Rust-side persistence, when the schema and database access are implemented, then SQLite access exists only under `src-tauri` and no frontend TypeScript module accesses SQLite directly.
4. Given Phase 1 excludes Node ORMs, when dependencies are inspected, then Prisma, Drizzle, Kysely, MikroORM, TypeORM, and Node ORM sidecars are absent.
5. Given migrations and repository initialization are complete, when a command-level or repository-level smoke test creates a task record, then the task can be read back from SQLite with its stable ID, text, status, timestamps, and nullable `deleted_at` fields intact.

## Tasks / Subtasks

- [ ] Add SQLx SQLite schema and migration assets. (AC: 1, 2, 3, 4)
  - [ ] Add Rust-side SQLx SQLite dependencies only under `src-tauri/Cargo.toml`.
  - [ ] Add `src-tauri/migrations/0001_create_tasks.sql`.
  - [ ] Ensure schema has `id`, `text`, `status`, `created_at`, `updated_at`, and nullable `deleted_at`.
  - [ ] Do not add frontend SQLite, Node ORM, backend, cloud, mobile, or sync dependencies.
- [ ] Add Rust migration and repository initialization. (AC: 1, 3)
  - [ ] Add `src-tauri/src/tasks/migrations.rs` to run embedded migrations.
  - [ ] Add `src-tauri/src/tasks/repository.rs` with SQLite pool creation and minimal create/read smoke support.
  - [ ] Add app startup initialization so migrations run during Tauri setup before future task commands can be used.
- [ ] Add Rust task model and app state boundary. (AC: 3, 5)
  - [ ] Add `src-tauri/src/tasks/model.rs`.
  - [ ] Add app state wiring under `src-tauri/src/app/state.rs`.
  - [ ] Keep TypeScript free of SQLite access.
- [ ] Perform story validation. (AC: 1-5)
  - [ ] Run `cargo fmt --check`.
  - [ ] Run or attempt Rust smoke test for repository create/read.
  - [ ] Run `pnpm build`.
  - [ ] Attempt `pnpm tauri dev` and record any external system blocker.
  - [ ] Inspect manifests for forbidden Node ORM/frontend persistence dependencies.

## Dev Notes

### Previous Story Intelligence

- Epic 1 is complete: Tauri scaffold, tray lifecycle, explicit quit, and compact panel shell are implemented.
- Rustup user toolchain is installed with `rustc 1.95.0`, `cargo 1.95.0`, and `rustfmt 1.9.0-stable`.
- Current native validation blocker is missing `dbus-1.pc`; install `libdbus-1-dev` and `pkg-config` before full Tauri validation.

### Architecture Guardrails

- SQLite access must stay inside `src-tauri`; frontend TypeScript must not access SQLite directly. [Source: _bmad-output/planning-artifacts/architecture.md#Pattern-Enforcement]
- SQLx is the approved Rust-side SQLite persistence layer. [Source: _bmad-output/planning-artifacts/architecture.md#Local-Storage-FR22-FR26]
- Schema changes must include migrations. [Source: _bmad-output/planning-artifacts/architecture.md#Pattern-Enforcement]
- Do not add Prisma, Drizzle, Kysely, MikroORM, TypeORM, or any Node ORM sidecar. [Source: _bmad-output/planning-artifacts/epics.md#Story-3-1-SQLx-SQLite-Task-Schema-and-Migrations]
- Use soft delete through `deleted_at` and avoid local time strings in the database. [Source: _bmad-output/planning-artifacts/architecture.md#Anti-Patterns]

### Expected Files to Touch

- `src-tauri/Cargo.toml`
- `src-tauri/migrations/0001_create_tasks.sql`
- `src-tauri/src/app/mod.rs`
- `src-tauri/src/app/state.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/src/tasks/mod.rs`
- `src-tauri/src/tasks/migrations.rs`
- `src-tauri/src/tasks/model.rs`
- `src-tauri/src/tasks/repository.rs`
- `_bmad-output/implementation-artifacts/sprint-status.yaml`

### References

- [Source: _bmad-output/planning-artifacts/epics.md#Story-3-1-SQLx-SQLite-Task-Schema-and-Migrations]
- [Source: _bmad-output/planning-artifacts/architecture.md#Local-Storage-FR22-FR26]
- [Source: _bmad-output/planning-artifacts/architecture.md#Pattern-Enforcement]

## Dev Agent Record

### Agent Model Used

TBD by dev agent.

### Debug Log References

### Completion Notes List

- Ultimate context engine analysis completed - comprehensive developer guide created.

### File List

- `_bmad-output/implementation-artifacts/3-1-sqlx-sqlite-task-schema-and-migrations.md`
