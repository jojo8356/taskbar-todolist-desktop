# Story 5.1: Mobile Local Task Companion

Status: ready-for-dev

<!-- Note: Validation is optional. Run validate-create-story for quality check before dev-story. -->

## Story

As a mobile user,
I want to view and manage the same simple task model on mobile,
so that I can carry my task list away from the desktop.

## Acceptance Criteria

1. Given the mobile companion is implemented in Phase 2, when the user opens it, then the user can view locally stored tasks.
2. Given the mobile user adds a task, when the task is saved, then it uses the same sync-ready fields as desktop: stable ID, text, status, created/updated timestamps, and deletion marker support.
3. Given the mobile app has no internet connection, when the user creates, edits, deletes, or views tasks, then base task actions work against mobile local storage.
4. Given the mobile user edits or deletes a task, when the action succeeds, then the local mobile store updates `updatedAt` and preserves deletion-marker semantics instead of hard-deleting records needed for later sync.
5. Given Story 5.1 scope is inspected, when implementation is complete, then no desktop-to-mobile sync protocol, conflict resolution, cloud account, backend service, public network dependency, project/tag/priority/calendar feature, or mandatory internet flow has been introduced.

## Tasks / Subtasks

- [ ] Decide and document the Flutter mobile scaffold location before coding. (AC: 1-5)
  - [ ] Prefer a clearly separated companion app boundary, for example a sibling `taskbar-todolist-mobile/` or `taskbar_todolist_mobile/` Flutter project.
  - [ ] If no Flutter project exists yet, create it with `flutter create` once the Flutter SDK is available.
  - [ ] Do not mix mobile-only UI code into the existing desktop tray/full-editor modules.
- [ ] Create the Flutter mobile companion task model and local persistence layer. (AC: 1-4)
  - [ ] Represent tasks with `id`, `text`, `status`, `createdAt`, `updatedAt`, and `deletedAt`.
  - [ ] Constrain `status` to `todo | done`.
  - [ ] Store timestamps as ISO 8601 UTC strings.
  - [ ] Soft-delete by setting `deletedAt` and `updatedAt`; active lists must filter deleted records.
- [ ] Implement local mobile task actions. (AC: 1-4)
  - [ ] Load active locally stored tasks on app open.
  - [ ] Create tasks offline with a stable generated ID and default `todo` status.
  - [ ] Edit task text and status offline.
  - [ ] Delete tasks offline through deletion-marker semantics.
  - [ ] Reject empty or whitespace-only task text.
- [ ] Build a minimal mobile UI for the same simple task workflow. (AC: 1-5)
  - [ ] Show locally stored active tasks.
  - [ ] Keep add/edit/delete/status controls intentionally simple.
  - [ ] Show compact local/offline status messaging.
  - [ ] Avoid project, tag, priority, calendar, collaboration, reminder, attachment, and advanced list concepts.
- [ ] Keep sync deferred behind clean boundaries. (AC: 2, 4, 5)
  - [ ] Ensure the persisted record shape can later be serialized for Story 5.3 transfer/merge.
  - [ ] Do not implement sync triggers, device discovery, conflict resolution, retry, USB transfer, or LAN transfer in this story.
- [ ] Add focused Flutter validation. (AC: 1-5)
  - [ ] Add unit tests for create/list/update/soft-delete/offline local storage behavior where the chosen mobile stack supports it.
  - [ ] Add a manual validation note proving the app can create, edit, delete, and reload tasks with network disabled.
  - [ ] Run `flutter test` for the mobile companion once the Flutter project is available.
  - [ ] Run the existing desktop checks after changes if this repo is touched: `cargo fmt --manifest-path src-tauri/Cargo.toml --check`, `cargo test --manifest-path src-tauri/Cargo.toml`, and `pnpm build`.

## Dev Notes

### Existing Desktop Model to Mirror

- Desktop TypeScript task type already uses the target Phase 2 shape: `id`, `text`, `status`, `createdAt`, `updatedAt`, `deletedAt`; `status` is limited to `"todo" | "done"` in `src/types/task.ts`. [Source: src/types/task.ts]
- Desktop Rust model stores the same fields in SQLite with snake_case persistence fields and UTC timestamps, then exposes camelCase DTO fields at the Tauri command boundary. [Source: src-tauri/src/tasks/model.rs] [Source: src-tauri/src/tasks/dto.rs]
- Desktop SQLite schema uses `id TEXT PRIMARY KEY`, `text`, constrained `status`, `created_at`, `updated_at`, and nullable `deleted_at`; active reads filter `deleted_at IS NULL`. [Source: src-tauri/migrations/0001_create_tasks.sql] [Source: src-tauri/src/tasks/repository.rs]
- Desktop soft-delete updates `deleted_at` and `updated_at`; do not hard-delete task records in the mobile companion because later sync stories need tombstones. [Source: src-tauri/src/tasks/repository.rs]
- Desktop validates text by trimming whitespace and rejecting empty task text. Mobile should match that behavior to avoid sync-time cleanup. [Source: src-tauri/src/tasks/repository.rs]

### Architecture Guardrails

- The product is desktop-first, local-first, account-free, and cloud-free; mobile is a companion surface, not a broader productivity suite. [Source: _bmad-output/planning-artifacts/prd.md#Project Overview] [Source: _bmad-output/planning-artifacts/prd.md#Product Scope]
- Epic 5 provides mobile and local/USB sync, but Story 5.1 is only the local mobile companion. Stories 5.2-5.4 own sync mode selection, explicit trigger, bidirectional transfer/merge, deletion propagation, failure feedback, and retry. [Source: _bmad-output/planning-artifacts/epics.md#Epic 5 Mobile Companion and Local Sync]
- Keep Phase 2 sync-related code behind explicit module boundaries. This story can prepare a serializable task record, but must not require sync to pass local mobile tests. [Source: _bmad-output/planning-artifacts/architecture.md#Enforcement Guidelines]
- Do not add backend, cloud sync, account/auth provider, analytics, telemetry, Prisma, Drizzle, Kysely, MikroORM, TypeORM, or another Node ORM runtime for this story. [Source: _bmad-output/planning-artifacts/architecture.md#Data Architecture] [Source: _bmad-output/planning-artifacts/architecture.md#External Integrations]
- Mobile implementation decision: use Flutter for the companion app. Tauri remains the desktop shell for this repo; do not add Tauri mobile targets unless the architecture is explicitly changed later.
- Tauri v2 can technically target Android/iOS, but this story should not use that path because the intended mobile stack is Flutter.

### Project Structure Notes

- No sibling Flutter mobile project was present next to this checkout during story creation, and the Flutter SDK was not installed in this environment when development was attempted. If the Flutter app exists elsewhere, connect to it explicitly before implementation; otherwise Story 5.1 should create the mobile companion boundary once Flutter is available.
- Existing desktop modules are organized as `src/ui/` for tray/full editor UI, `src/state/` for frontend task state and command wrappers, and `src-tauri/src/tasks/` for Rust persistence. Do not put mobile UI into desktop tray modules like `src/ui/tray-panel.ts` or desktop full-editor modules like `src/ui/full-edit.ts`. [Source: _bmad-output/planning-artifacts/architecture.md#Project Structure & Boundaries]
- If a Flutter project is created inside this repo instead of as a sibling, keep it under a clear isolated path such as `mobile/` and preserve current desktop entry points.

### UX Requirements

- Keep the mobile experience minimal: view active tasks, add, edit, delete, and change `todo`/`done` status locally. [Source: _bmad-output/planning-artifacts/epics.md#Story 5.1 Mobile Local Task Companion]
- The user must feel tasks are local and usable without internet; local/offline status messaging should be compact and not block task capture. [Source: _bmad-output/planning-artifacts/ux-design-specification.md#UX Principles]
- Sync controls, sync progress, success/failure messages, retry, and conflict handling are not Story 5.1 UI. Reserve them for Stories 5.2-5.4. [Source: _bmad-output/planning-artifacts/epics.md#Story 5.2 Sync Mode Selection and Explicit Trigger] [Source: _bmad-output/planning-artifacts/epics.md#Story 5.4 Deletion Propagation Sync Feedback and Retry]

### Previous Story Intelligence

- Epic 4 completed the full edit UI by reusing the existing persisted `update_task` command and shared task source; Story 5.1 should follow the same discipline: one local source of truth per platform and no duplicate parallel task state. [Source: _bmad-output/implementation-artifacts/4-3-change-task-status-in-full-ui.md]
- Recent validation pattern from Story 4.3 was `cargo fmt --manifest-path src-tauri/Cargo.toml --check`, `cargo test --manifest-path src-tauri/Cargo.toml`, and `pnpm build`; keep these checks when desktop files are modified. [Source: _bmad-output/implementation-artifacts/4-3-change-task-status-in-full-ui.md]
- Git history shows completed desktop epics for scaffold/tray lifecycle, persistence, tray task workflow, and full edit UI. Treat the desktop app as stable baseline and avoid regressions while adding mobile companion work.

### Testing Standards Summary

- Test local persistence semantics directly: create, list active newest-first or chosen mobile order, update text/status, reject empty text, soft-delete, hide deleted tasks from active list, and survive app restart/reload.
- Include a manual offline validation: disable network, open mobile app, create a task, edit it, mark it done, delete it, restart/reopen, and confirm the expected local state remains.
- If no automated Flutter test harness exists yet, document that gap in the Dev Agent Record and still add tests around the pure Dart persistence/model layer.

### References

- [Source: _bmad-output/planning-artifacts/epics.md#Epic 5 Mobile Companion and Local Sync]
- [Source: _bmad-output/planning-artifacts/prd.md#Mobile Companion]
- [Source: _bmad-output/planning-artifacts/prd.md#Synchronization]
- [Source: _bmad-output/planning-artifacts/architecture.md#Data Architecture]
- [Source: _bmad-output/planning-artifacts/architecture.md#Enforcement Guidelines]
- [Source: _bmad-output/planning-artifacts/ux-design-specification.md#UX Principles]
- [Source: src/types/task.ts]
- [Source: src-tauri/src/tasks/repository.rs]
- [Source: src-tauri/migrations/0001_create_tasks.sql]

## Dev Agent Record

### Agent Model Used

{{agent_model_name_version}}

### Debug Log References

### Completion Notes List

### File List
