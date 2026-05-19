---
stepsCompleted:
  - 1
  - 2
  - 3
  - 4
  - 5
  - 6
  - 7
  - 8
inputDocuments:
  - _bmad-output/planning-artifacts/prd.md
  - _bmad-output/planning-artifacts/product-brief-taskbar-todolist.md
  - _bmad-output/planning-artifacts/ux-design-specification.md
  - _bmad-output/planning-artifacts/ux-design-directions.html
  - _bmad-output/planning-artifacts/validation-report-prd.md
  - docs/product.md
workflowType: "architecture"
project_name: "taskbar-todolist-desktop"
user_name: "Johan"
date: "2026-05-19"
lastStep: 8
status: "complete"
completedAt: "2026-05-19"
---

# Architecture Decision Document

_This document builds collaboratively through step-by-step discovery. Sections are appended as we work through each architectural decision together._

## Project Context Analysis

### Requirements Overview

**Functional Requirements:**
The product has 45 functional requirements. Phase 1 centers on a Linux desktop app with a system tray entry point, quick task capture, quick deletion, full editing UI, local storage, offline operation, and minimal settings/control. The core Phase 1 architecture must support:

- creating, listing, editing, completing, and deleting simple tasks;
- a tray icon that opens a compact quick panel;
- an input fixed at the top of the tray panel with `Enter` add behavior;
- a task list below the input with row-level trash actions;
- a secondary full edit UI for text/status modification;
- local persistence and startup loading;
- background availability through the tray;
- quitting the app explicitly.

Phase 2 adds mobile and local/USB sync. Architecture should prepare the task data model for sync through stable IDs, timestamps, status, and deletion markers, but should not require mobile, sync, backend, or network services for Phase 1.

**Non-Functional Requirements:**
The NFRs that shape architecture are:

- tray panel opens in under 1 second with 500 local tasks;
- local list loading finishes in under 500 ms for 500 tasks;
- background idle usage stays under 150 MB RSS and under 2% average CPU over 5 minutes without sync;
- local changes survive app close/reopen in 100% of MVP manual tests;
- sync failures must not delete or corrupt local data;
- no account, cloud, or internet dependency for MVP operation;
- explicit sync trigger/acceptance for MVP sync;
- Tauri/Linux tray behavior must be proven before mobile or sync work;
- Phase 1 must remain implementable without backend, app mobile, or external service.

**Scale & Complexity:**

- Primary domain: Linux desktop productivity utility.
- Complexity level: low-to-medium.
- Estimated architectural components: 7 core components.

Likely components:

1. Tauri app shell and lifecycle.
2. Linux tray/system integration.
3. Quick tray panel UI.
4. Full editing window UI.
5. Shared task state/store.
6. Local persistence layer.
7. Future sync boundary/interfaces.

### Technical Constraints & Dependencies

Known constraints:

- Desktop technology is Tauri.
- Primary MVP platform is Linux.
- The architecture must choose and document a concrete Linux validation baseline for tray/AppIndicator support.
- Phase 1 must work offline and without account/cloud/backend.
- The tray prototype is the first technical risk and should be front-loaded.
- Local persistence should use a structured model suitable for future sync.
- SQLite is recommended by the PRD, but the architecture should make the final persistence decision explicitly.
- UX direction is Compact Native: small panel, top input, list below, row trash action, compact spacing, native-feeling typography, light theme first.

### Persistence Technology Decision Notes

Phase 1 persistence should use SQLite from the Rust/Tauri side, with SQLx as the preferred database access layer.

Decision:

- Use Rust-side SQLite persistence with SQLx.
- Expose task operations to the frontend through Tauri commands.
- Do not use Prisma, Drizzle, Kysely, MikroORM, TypeORM, or other Node/TypeScript ORMs in Phase 1.
- Do not add a Node sidecar just to run a Prisma-like ORM.

Rationale:

- Tauri does not provide a normal Node runtime inside the frontend, so Node-centric SQLite ORMs fit Electron better than Tauri.
- SQLx works naturally in `src-tauri`, keeps persistence native, and avoids an extra runtime process.
- The task model is small enough that a full entity ORM is unnecessary.
- Rust-side persistence keeps local data safety, migrations, and command boundaries close to the native shell.
- This supports the Phase 2 sync-ready data model without forcing sync implementation into Phase 1.

Alternatives considered:

| Option | Fit for Tauri SQLite | Prisma-like DX | Runtime weight | Decision |
| --- | --- | --- | --- | --- |
| SQLx | Excellent | Medium | Low | Selected |
| Diesel | Good | Medium | Medium | Possible fallback if a fuller Rust ORM is needed |
| SeaORM | Medium | Good | Higher | Too web-service oriented for MVP |
| Drizzle + better-sqlite3 | Poor in pure Tauri | Good | Requires Node runtime | Rejected for Phase 1 |
| Prisma SQLite | Poor in pure Tauri | Excellent | Requires Prisma/client runtime fit | Rejected for Phase 1 |
| Kysely | Poor in pure Tauri | Good | Node-centric | Rejected for Phase 1 |
| MikroORM | Poor in pure Tauri | Good | Higher | Rejected for Phase 1 |
| TypeORM | Poor in pure Tauri | Medium | Higher | Rejected for Phase 1 |

Initial persistence module shape:

```text
src-tauri/src/tasks/
  model.rs
  repository.rs
  migrations.rs
  commands.rs
```

Frontend command boundary:

```ts
await invoke("create_task", { text })
await invoke("list_tasks")
await invoke("update_task", { id, text, status })
await invoke("delete_task", { id })
```

Sources checked:

- Tauri SQL plugin: https://v2.tauri.app/plugin/sql/
- SQLx SQLite: https://docs.rs/sqlx/latest/sqlx/sqlite/
- Diesel: https://diesel.rs/
- SeaORM: https://www.sea-ql.org/SeaORM/docs/index/
- Drizzle SQLite: https://orm.drizzle.team/docs/get-started-sqlite
- Prisma SQLite: https://docs.prisma.io/orm/core-concepts/supported-databases/sqlite
- Kysely: https://www.kysely.dev/
- MikroORM SQLite: https://mikro-orm.io/docs/usage-with-sql
- TypeORM SQLite: https://typeorm.io/docs/drivers/sqlite

### Cross-Cutting Concerns Identified

- **Tray lifecycle:** app must stay alive in background, panel close must not quit the process, explicit quit must exist.
- **Focus and keyboard behavior:** tray panel opens with input focused; `Enter` adds and preserves focus.
- **State consistency:** tray panel and full edit UI must share the same source of truth.
- **Local data safety:** task changes must persist immediately and survive restart.
- **Deletion semantics:** deleted tasks need representation for future sync propagation.
- **Performance:** local reads and tray opening must remain fast with 500 tasks.
- **Scope control:** no projects, tags, priorities, calendar, collaboration, mandatory cloud, or backend in Phase 1.
- **Future sync readiness:** task model and storage should support Phase 2 without forcing Phase 2 implementation now.
- **UI consistency:** tray panel and full UI should follow the UX spec's compact native design system.

## Starter Template Evaluation

### Primary Technology Domain

Desktop app, based on the project requirements analysis.

The project is a Linux-first Tauri desktop utility with a compact tray panel, full editing window, local persistence, and future sync readiness. Phase 1 does not need a backend, account system, cloud service, routing framework, server rendering, or mobile app scaffold.

### Starter Options Considered

**Official Tauri Vanilla TypeScript**

This starter provides the lightest official Tauri foundation: Rust/Tauri shell, Vite-based frontend, TypeScript frontend option, and minimal UI assumptions. It fits the product because the Phase 1 UI is intentionally small: tray panel, task input, task rows, row delete action, and full edit window.

Trade-offs:

- Best fit for validating the Linux tray risk early.
- Keeps frontend architecture simple.
- Avoids React/Vue/Svelte decisions before the core desktop behavior is proven.
- Requires manually defining lightweight component/module conventions.

**Official Tauri React TypeScript**

This starter would provide a familiar component model for `TrayPanel`, `TaskInput`, `TaskRow`, and `FullEditWindow`.

Trade-offs:

- Better if the UI grows into a richer app later.
- More dependencies and framework conventions than Phase 1 requires.
- Can be introduced later if Vanilla TypeScript becomes limiting.

**Manual Vite + Tauri Setup**

Manual setup would allow maximum control but adds unnecessary setup decisions before the tray prototype is proven.

Trade-offs:

- Flexible, but not needed.
- Higher chance of inconsistent setup.
- Less aligned with using official maintained templates.

### Selected Starter: Official Tauri Vanilla TypeScript

**Rationale for Selection:**

Use the official Tauri Vanilla TypeScript starter for Phase 1. The architecture should optimize for proving Tauri Linux tray behavior, local persistence, and the compact interaction loop before introducing frontend framework complexity.

The UX spec describes components, but the interaction surface is small enough to implement with TypeScript modules and explicit DOM/state boundaries. Tailwind can be added for the compact native design system without requiring a larger UI framework.

Persistence is intentionally not handled by a Node/TypeScript ORM. SQLite persistence belongs in `src-tauri` with SQLx, exposed to the frontend through Tauri commands.

**Initialization Command:**

```bash
pnpm create tauri-app taskbar-todolist-desktop
```

Prompt selections:

```text
Frontend language: TypeScript / JavaScript
Package manager: pnpm
UI template: Vanilla
UI flavor: TypeScript
```

Then run:

```bash
cd taskbar-todolist-desktop
pnpm install
pnpm tauri dev
```

**Architectural Decisions Provided by Starter:**

**Language & Runtime:**

- Rust backend through Tauri.
- TypeScript frontend.
- Native desktop shell controlled by Tauri.
- Frontend served through Vite during development.

**Styling Solution:**

The starter does not impose the final styling system. Add Tailwind CSS after initialization to implement the UX spec's compact design tokens, spacing, colors, and utility-first styling.

**Build Tooling:**

- Vite frontend build pipeline.
- Tauri development and build commands.
- Rust/Cargo native app build pipeline.

**Testing Framework:**

The starter does not make a complete testing decision. Architecture should define testing separately:

- Rust unit tests for task repository and persistence behavior.
- Frontend unit tests for task state and UI event behavior.
- Manual MVP validation for Linux tray behavior.
- Later E2E or scripted smoke tests once the tray prototype is stable.

**Code Organization:**

The starter should be evolved into these boundaries:

- `src/main.ts` for frontend bootstrapping only.
- `src/ui/` for tray panel and full edit UI modules.
- `src/state/` for task state and commands called by UI.
- `src-tauri/src/` for Tauri setup, tray lifecycle, commands, persistence, and app lifecycle.
- `src-tauri/src/tasks/` for task model, repository, migrations, commands, and SQLx persistence logic.

**Development Experience:**

- Use `pnpm tauri dev` for local development.
- First implementation story should scaffold the app and prove tray creation on the target Linux environment.
- Add Tailwind only after the base Tauri app runs.
- Add SQLite/SQLx persistence only after tray lifecycle is proven.

**Additional Setup Decisions Deferred From Starter:**

- Tailwind CSS setup.
- SQLx migration setup.
- Exact Linux validation baseline for tray/AppIndicator.
- Test framework selection.
- Packaging strategy.

**Note:** Project initialization using this command should be the first implementation story.

## Core Architectural Decisions

### Decision Priority Analysis

**Critical Decisions (Block Implementation):**

- Use Tauri v2 as the desktop shell.
- Use the official Tauri Vanilla TypeScript starter.
- Use Rust-side SQLite persistence with SQLx.
- Expose all task persistence operations through Tauri commands.
- Validate Linux tray/AppIndicator behavior before implementing full persistence, mobile, or sync.
- Keep Phase 1 offline-only and backend-free.

**Important Decisions (Shape Architecture):**

- Use a compact TypeScript frontend with explicit modules instead of a frontend framework in Phase 1.
- Use Tailwind CSS v4 for the compact native UI system after base Tauri startup is proven.
- Keep tray panel UI and full edit UI separate but backed by the same task command/state boundary.
- Use a sync-ready task model from Phase 1: stable ID, text, status, created/updated timestamps, and deletion marker.
- Use soft-delete/tombstone semantics internally for deleted tasks, while hiding deleted tasks from active lists.

**Deferred Decisions (Post-MVP):**

- Mobile app framework.
- Local network or USB sync protocol.
- Packaging and auto-update strategy.
- Multi-desktop-environment support beyond the MVP Linux validation baseline.
- Recovery/trash UX beyond immediate deletion semantics.

### Data Architecture

**Database:**
SQLite local file, accessed from Rust through SQLx.

**Rationale:**
SQLite matches the local-first/offline requirement. SQLx fits Tauri because persistence lives naturally in `src-tauri`, avoids a Node sidecar, and keeps data safety close to the native command boundary.

**Rejected for Phase 1:**
Prisma, Drizzle, Kysely, MikroORM, TypeORM, and other Node/TypeScript ORMs. These fit Node/Electron-style runtime assumptions better than pure Tauri.

**Task Model:**

```text
Task
- id: stable string ID
- text: string
- status: todo | done
- created_at: timestamp
- updated_at: timestamp
- deleted_at: nullable timestamp
```

**Migration Approach:**
Use SQLx migrations under `src-tauri/migrations` or an equivalent Rust-side migrations folder. Migrations must run during app startup before task commands become available.

**Repository Boundary:**

```text
src-tauri/src/tasks/
  model.rs
  repository.rs
  migrations.rs
  commands.rs
```

**Caching Strategy:**
No separate cache in Phase 1. The frontend may hold an in-memory task list for UI rendering, but SQLite remains the source of truth.

### Authentication & Security

**Authentication:**
None in Phase 1.

**Authorization:**
None in Phase 1 because there is no multi-user model.

**Data Location:**
Store the SQLite database in the app data directory managed from the Tauri side.

**Cloud/Data Egress:**
No cloud sync, account, analytics, telemetry, or default external data transfer in Phase 1.

**Data Safety:**
Task writes must be transactional. Deletes are represented with `deleted_at` rather than destructive removal so Phase 2 sync can propagate deletions.

### API & Communication Patterns

**Frontend-to-Backend Boundary:**
Use Tauri commands as the only frontend persistence boundary.

Initial commands:

```ts
await invoke("create_task", { text })
await invoke("list_tasks")
await invoke("update_task", { id, text, status })
await invoke("delete_task", { id })
```

**Error Handling:**
Rust commands return typed success/error results. Frontend maps errors to compact status messages.

**External API:**
No HTTP/REST/GraphQL API in Phase 1.

**Future Sync Boundary:**
Define sync as a future service boundary behind Rust-side modules. Phase 1 data model must support sync, but Phase 1 implementation must not depend on sync.

### Frontend Architecture

**Framework:**
Vanilla TypeScript with explicit modules for Phase 1.

**Rationale:**
The UI is intentionally small. Avoiding React/Vue/Svelte keeps attention on tray behavior, persistence, and the core add/delete loop.

**UI Modules:**

```text
src/
  main.ts
  ui/
    tray-panel.ts
    task-input.ts
    task-list.ts
    task-row.ts
    full-edit-window.ts
    status-message.ts
  state/
    task-store.ts
    task-commands.ts
  styles/
    main.css
```

**State Management:**
Small custom task store in TypeScript. It mirrors task data returned by Tauri commands but does not replace SQLite as source of truth.

**Styling:**
Tailwind CSS v4 after Tauri startup is proven. Use UX spec tokens: compact spacing, low radius, light theme first, red only for destructive actions.

**Routing:**
No router in Phase 1. The app has a tray panel and full edit window, not a multi-page application.

### Infrastructure & Deployment

**Hosting:**
None.

**Build/Runtime:**
Tauri build pipeline with Vite frontend and Rust backend.

**Development Commands:**

```bash
pnpm create tauri-app taskbar-todolist-desktop
pnpm install
pnpm tauri dev
```

**Validation Baseline:**
Architecture must name one MVP Linux desktop environment with working tray/AppIndicator support before implementation stories begin.

**Implementation Sequence:**

1. Scaffold Tauri Vanilla TypeScript app.
2. Prove Linux tray icon, panel opening, focus behavior, and background lifecycle.
3. Add compact tray UI skeleton.
4. Add Rust task model and SQLx SQLite migrations.
5. Add Tauri task commands.
6. Connect frontend task store to Tauri commands.
7. Add full edit window.
8. Add persistence/restart validation.
9. Defer mobile/sync.

### Decision Impact Analysis

**Implementation Sequence:**
The tray prototype comes before persistence because system tray reliability is the highest technical risk. SQLx persistence comes before full edit polish because state consistency and restart safety are core NFRs.

**Cross-Component Dependencies:**

- Tray panel depends on Tauri lifecycle and frontend task store.
- Full edit window depends on the same task command boundary as the tray panel.
- SQLx repository depends on migrations and task model.
- Future sync depends on stable IDs, timestamps, and deletion markers.
- UI consistency depends on shared CSS/Tailwind tokens and compact component conventions.

## Implementation Patterns & Consistency Rules

### Pattern Categories Defined

**Critical Conflict Points Identified:**
Nine areas where AI agents could make incompatible choices:

1. Database naming.
2. Rust model naming.
3. TypeScript model naming.
4. Tauri command naming and payloads.
5. Soft-delete behavior.
6. Error response format.
7. Date/time format.
8. Frontend state ownership.
9. File and test organization.

### Naming Patterns

**Database Naming Conventions:**

Use SQLite snake_case names.

Rules:

- Tables: plural snake_case, e.g. `tasks`.
- Columns: snake_case, e.g. `created_at`, `deleted_at`.
- Primary key: `id`.
- Indexes: `idx_<table>_<columns>`, e.g. `idx_tasks_deleted_at`.
- Migrations: ordered SQLx-compatible migration names.

Task table baseline:

```sql
CREATE TABLE tasks (
  id TEXT PRIMARY KEY,
  text TEXT NOT NULL,
  status TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  deleted_at TEXT
);
```

**Rust Naming Conventions:**

Use Rust idioms.

Rules:

- Types: `PascalCase`, e.g. `Task`, `TaskStatus`, `TaskRepository`.
- Functions: `snake_case`, e.g. `create_task`, `list_tasks`.
- Fields: `snake_case`, matching database fields where practical.
- Tauri command functions: snake_case and action-oriented.

Rust model baseline:

```rust
pub struct Task {
    pub id: String,
    pub text: String,
    pub status: TaskStatus,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}
```

**TypeScript Naming Conventions:**

Use TypeScript idioms at the frontend boundary.

Rules:

- Types/interfaces: `PascalCase`, e.g. `Task`, `TaskStatus`.
- Functions: `camelCase`, e.g. `createTask`, `listTasks`.
- Fields exposed to UI: `camelCase`, e.g. `createdAt`, `deletedAt`.
- Files: kebab-case, e.g. `task-store.ts`, `tray-panel.ts`.

Frontend model baseline:

```ts
export type TaskStatus = "todo" | "done";

export interface Task {
  id: string;
  text: string;
  status: TaskStatus;
  createdAt: string;
  updatedAt: string;
  deletedAt: string | null;
}
```

### Structure Patterns

**Project Organization:**

Rust/Tauri:

```text
src-tauri/src/
  main.rs
  tasks/
    mod.rs
    model.rs
    repository.rs
    migrations.rs
    commands.rs
  app/
    tray.rs
    windows.rs
    errors.rs
```

Frontend:

```text
src/
  main.ts
  ui/
    tray-panel.ts
    task-input.ts
    task-list.ts
    task-row.ts
    full-edit-window.ts
    status-message.ts
  state/
    task-store.ts
    task-commands.ts
  styles/
    main.css
```

**File Structure Patterns:**

- Domain logic lives in `src-tauri/src/tasks/`.
- Tray lifecycle logic lives in `src-tauri/src/app/tray.rs`.
- Window creation and show/hide behavior lives in `src-tauri/src/app/windows.rs`.
- Frontend modules must not access SQLite directly.
- Frontend modules must call `state/task-commands.ts`, not raw `invoke(...)` everywhere.

**Test Organization:**

Rust:

- Unit tests live near the module under `#[cfg(test)]`.
- Persistence integration tests can live under `src-tauri/tests/` once the schema stabilizes.

TypeScript:

- UI/state unit tests should use `*.test.ts` next to the module or under `src/**/*.test.ts`.
- Do not add browser E2E tests until tray behavior is proven manually.

### Format Patterns

**Tauri Command Response Formats:**

Rust commands should return `Result<T, AppError>`.

Frontend command wrappers convert Rust response/error into UI-friendly results.

TypeScript wrapper pattern:

```ts
export async function createTask(text: string): Promise<Task> {
  return invoke<Task>("create_task", { text });
}
```

**Error Format:**

Rust app errors should include:

```text
code: stable machine-readable string
message: human-readable summary
```

Initial error codes:

```text
TASK_TEXT_EMPTY
TASK_NOT_FOUND
TASK_STORAGE_ERROR
TASK_VALIDATION_ERROR
APP_INTERNAL_ERROR
```

Frontend rule:

- Show compact user-facing messages.
- Log technical details only in development.
- Do not expose raw SQL or Rust debug errors in UI text.

**Date/Time Format:**

Use ISO 8601 UTC strings across Rust <-> TypeScript and SQLite text columns.

Rules:

- Store timestamps as UTC ISO strings.
- Never store localized display strings in SQLite.
- Format for display only in the frontend UI layer.

**Data Exchange Format:**

- Rust/database fields may be snake_case internally.
- Tauri command DTOs exposed to TypeScript should use camelCase.
- Do conversion at the command boundary, not throughout UI modules.

### Communication Patterns

**Tauri Command Naming:**

Use action-oriented snake_case:

```text
create_task
list_tasks
update_task
delete_task
```

Future commands should follow:

```text
verb_noun
```

Examples:

```text
open_full_edit_window
close_tray_panel
get_app_status
```

**Command Payloads:**

Payloads must be minimal and explicit.

```ts
create_task: { text: string }
update_task: { id: string; text?: string; status?: TaskStatus }
delete_task: { id: string }
list_tasks: no payload
```

**State Management Patterns:**

- SQLite is the source of truth.
- `task-store.ts` is a frontend cache for rendering and UI responsiveness.
- After a mutation command succeeds, update the frontend store from the command result or reload the list.
- Do not let UI-only state mutate task records without a successful Tauri command.

### Process Patterns

**Delete Pattern:**

User-facing delete removes the task from active lists immediately after command success.

Implementation:

- Set `deleted_at`, do not hard-delete.
- Active task queries filter `deleted_at IS NULL`.
- Future sync can propagate tombstones.

**Loading State Patterns:**

Use local loading states only.

Examples:

```ts
isAddingTask
isLoadingTasks
deletingTaskIds: Set<string>
```

Rules:

- No global loading overlay for tray actions.
- Adding should keep input focused.
- Empty input + `Enter` does nothing.

**Error Handling Patterns:**

- Validation errors return stable error codes.
- Storage errors become compact status messages.
- Failed mutations must not update the frontend task store as if they succeeded.
- Sync errors are Phase 2 and must not affect Phase 1 task operations.

### Enforcement Guidelines

**All AI Agents MUST:**

- Keep SQLite access inside `src-tauri`.
- Use SQLx for SQLite persistence.
- Use Tauri commands as the frontend/backend boundary.
- Use soft delete through `deleted_at`.
- Keep Phase 1 free of backend, account, cloud, and sync dependencies.
- Use snake_case for database/Rust command names and camelCase for TypeScript UI fields.
- Keep tray panel and full edit UI backed by the same task command/store boundary.
- Preserve input focus after successful add.
- Avoid introducing project/tag/priority/calendar concepts.

**Pattern Enforcement:**

- Any new task persistence function must be added to `repository.rs` and exposed through `commands.rs`.
- Any new frontend task operation must go through `state/task-commands.ts`.
- Any schema change must include a migration and update Rust/TypeScript task types.
- Any user-facing error must map from a stable error code.
- Any Phase 2 sync-related code must remain behind explicit module boundaries and must not be required for Phase 1 tests.

### Pattern Examples

**Good Examples:**

```rust
#[tauri::command]
pub async fn create_task(text: String, state: State<'_, AppState>) -> Result<TaskDto, AppError> {
    state.tasks.create(text).await.map(TaskDto::from)
}
```

```ts
const task = await createTask(input.value);
taskStore.add(task);
input.value = "";
input.focus();
```

```sql
SELECT id, text, status, created_at, updated_at, deleted_at
FROM tasks
WHERE deleted_at IS NULL
ORDER BY created_at DESC;
```

**Anti-Patterns:**

- Calling SQLite from frontend TypeScript.
- Adding Prisma/Drizzle/Kysely to Phase 1.
- Hard-deleting rows from `tasks`.
- Using local time strings in the database.
- Duplicating task state separately in tray and full edit UI.
- Letting `deleteTask()` update UI before command success.
- Adding a backend service for Phase 1.
- Adding project/tag/priority concepts to the MVP data model.

## Project Structure & Boundaries

### Complete Project Directory Structure

```text
taskbar-todolist-desktop/
├── README.md
├── package.json
├── pnpm-lock.yaml
├── tsconfig.json
├── vite.config.ts
├── index.html
├── .gitignore
├── .env.example
├── src/
│   ├── main.ts
│   ├── ui/
│   │   ├── tray-panel.ts
│   │   ├── task-input.ts
│   │   ├── task-list.ts
│   │   ├── task-row.ts
│   │   ├── full-edit-window.ts
│   │   └── status-message.ts
│   ├── state/
│   │   ├── task-store.ts
│   │   └── task-commands.ts
│   ├── types/
│   │   └── task.ts
│   └── styles/
│       └── main.css
├── src-tauri/
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── capabilities/
│   │   └── default.json
│   ├── migrations/
│   │   └── 0001_create_tasks.sql
│   ├── icons/
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs
│   │   ├── app/
│   │   │   ├── mod.rs
│   │   │   ├── tray.rs
│   │   │   ├── windows.rs
│   │   │   ├── state.rs
│   │   │   └── errors.rs
│   │   └── tasks/
│   │       ├── mod.rs
│   │       ├── model.rs
│   │       ├── repository.rs
│   │       ├── migrations.rs
│   │       ├── commands.rs
│   │       └── dto.rs
│   └── tests/
│       └── task_repository.rs
├── docs/
│   └── product.md
└── _bmad-output/
    └── planning-artifacts/
        ├── prd.md
        ├── product-brief-taskbar-todolist.md
        ├── ux-design-specification.md
        ├── ux-design-directions.html
        ├── validation-report-prd.md
        └── architecture.md
```

### Architectural Boundaries

**API Boundaries:**

There is no HTTP API in Phase 1.

The only application boundary between frontend and persistence is Tauri commands:

```text
TypeScript UI -> state/task-commands.ts -> Tauri invoke -> Rust commands.rs -> repository.rs -> SQLite
```

Initial command boundary:

```text
create_task
list_tasks
update_task
delete_task
```

**Component Boundaries:**

Frontend UI modules are small and explicit:

- `tray-panel.ts` owns tray panel rendering and composition.
- `task-input.ts` owns input behavior and `Enter` add interaction.
- `task-list.ts` owns active task list rendering.
- `task-row.ts` owns one task row and delete action wiring.
- `full-edit-window.ts` owns the secondary edit surface.
- `status-message.ts` owns compact user-facing feedback.

Frontend UI modules must not call `invoke(...)` directly. They call functions from `state/task-commands.ts`.

**Service Boundaries:**

Rust service boundaries:

- `app/tray.rs`: tray icon, menu, click handling, show/hide behavior.
- `app/windows.rs`: window creation and lifecycle.
- `app/state.rs`: shared app state, including task repository handle.
- `app/errors.rs`: `AppError` and error mapping.
- `tasks/model.rs`: Rust task domain types.
- `tasks/dto.rs`: command-facing DTOs.
- `tasks/repository.rs`: SQLx persistence operations.
- `tasks/migrations.rs`: migration execution.
- `tasks/commands.rs`: Tauri command functions.

**Data Boundaries:**

SQLite is accessed only from Rust.

- SQLite schema lives in `src-tauri/migrations/`.
- SQLx queries live in `tasks/repository.rs`.
- Tauri command DTO conversion lives in `tasks/dto.rs`.
- TypeScript task types live in `src/types/task.ts`.

### Requirements to Structure Mapping

**Task Management FR1-FR8**

- Rust model: `src-tauri/src/tasks/model.rs`
- Rust repository: `src-tauri/src/tasks/repository.rs`
- DTOs: `src-tauri/src/tasks/dto.rs`
- Frontend type: `src/types/task.ts`
- UI rendering: `src/ui/task-list.ts`, `src/ui/task-row.ts`

**Taskbar / Tray Experience FR9-FR16**

- Tray lifecycle: `src-tauri/src/app/tray.rs`
- Window lifecycle: `src-tauri/src/app/windows.rs`
- Tray panel UI: `src/ui/tray-panel.ts`
- Input behavior: `src/ui/task-input.ts`
- List/delete UI: `src/ui/task-list.ts`, `src/ui/task-row.ts`

**Full Editing UI FR17-FR21**

- Full edit window lifecycle: `src-tauri/src/app/windows.rs`
- Full edit UI: `src/ui/full-edit-window.ts`
- Shared task state: `src/state/task-store.ts`
- Task commands: `src/state/task-commands.ts`

**Local Storage FR22-FR26**

- SQLite migration: `src-tauri/migrations/0001_create_tasks.sql`
- Migration runner: `src-tauri/src/tasks/migrations.rs`
- SQLx repository: `src-tauri/src/tasks/repository.rs`
- Persistence tests: `src-tauri/tests/task_repository.rs`

**Mobile Companion / Sync FR27-FR41**

Deferred to Phase 2.

Phase 1 must still support sync readiness through:

- stable `id`;
- `created_at`;
- `updated_at`;
- `deleted_at`;
- soft-delete query behavior.

**Settings and Control FR42-FR45**

- Tray menu/control: `src-tauri/src/app/tray.rs`
- App/window lifecycle: `src-tauri/src/app/windows.rs`
- Status display: `src/ui/status-message.ts`

### Integration Points

**Internal Communication:**

```text
User action
-> UI module
-> task-store/task-commands
-> Tauri command
-> Rust repository
-> SQLite
-> DTO result
-> task-store update
-> UI render
```

**External Integrations:**

None in Phase 1.

Explicitly excluded:

- backend service;
- cloud sync;
- account/auth provider;
- analytics/telemetry;
- Prisma/Drizzle/Node ORM runtime.

**Data Flow:**

Create task:

```text
task-input.ts
-> createTask(text)
-> invoke("create_task")
-> commands.rs
-> repository.rs
-> INSERT INTO tasks
-> TaskDto
-> task-store.add(task)
-> tray-panel rerender
```

Delete task:

```text
task-row.ts
-> deleteTask(id)
-> invoke("delete_task")
-> commands.rs
-> repository.rs
-> UPDATE tasks SET deleted_at = ...
-> task-store.removeFromActive(id)
-> task-list rerender
```

Load tasks:

```text
tray-panel.ts opens
-> listTasks()
-> invoke("list_tasks")
-> repository.rs
-> SELECT active tasks
-> task-store.replaceAll(tasks)
-> task-list render
```

### File Organization Patterns

**Configuration Files:**

- `package.json`: frontend scripts and package dependencies.
- `vite.config.ts`: Vite frontend config.
- `tsconfig.json`: TypeScript config.
- `src-tauri/Cargo.toml`: Rust dependencies.
- `src-tauri/tauri.conf.json`: app shell, windows, bundle config.
- `src-tauri/capabilities/default.json`: Tauri command permissions.
- `.env.example`: documented environment variables, if any are introduced.

**Source Organization:**

- `src/` contains frontend-only TypeScript and CSS.
- `src-tauri/src/` contains Rust-only native logic.
- `src-tauri/src/tasks/` is the task domain boundary.
- `src-tauri/src/app/` is the desktop shell/lifecycle boundary.
- `src/types/` mirrors command DTOs for frontend usage.

**Test Organization:**

- Rust unit tests live near modules.
- Rust repository integration tests live in `src-tauri/tests/`.
- TypeScript unit tests should be co-located as `*.test.ts` once a test runner is selected.
- No E2E suite until tray lifecycle is proven.

**Asset Organization:**

- App icons live in `src-tauri/icons/`.
- UI styling lives in `src/styles/main.css`.
- No large visual asset system is required for Phase 1.

### Development Workflow Integration

**Development Server Structure:**

Use the Tauri dev workflow:

```bash
pnpm tauri dev
```

Frontend runs through Vite. Native shell runs through Tauri/Cargo.

**Build Process Structure:**

Use Tauri build pipeline:

```bash
pnpm tauri build
```

Packaging decisions are post-MVP unless needed for local validation.

**Deployment Structure:**

No hosted deployment exists in Phase 1.

Distribution is local desktop binary/package only, handled by Tauri when packaging is introduced.

## Architecture Validation Results

### Coherence Validation

**Decision Compatibility:**
The architectural decisions are compatible:

- Tauri v2 supports the selected desktop app model.
- Vanilla TypeScript matches the compact UI scope.
- Vite is supplied by the Tauri starter and supports the frontend build.
- SQLx + SQLite fits Rust-side persistence in `src-tauri`.
- Tauri commands provide a clean frontend/backend boundary.
- No backend/cloud/account dependency conflicts with the offline MVP requirement.

No contradictory decisions were found.

**Pattern Consistency:**
Implementation patterns support the decisions:

- SQLite access is constrained to Rust.
- Frontend command wrappers prevent scattered `invoke(...)` calls.
- snake_case is used for Rust/database names, camelCase for TypeScript UI models.
- Soft-delete semantics support future sync without requiring sync now.
- Error codes and DTO boundaries give agents a consistent command contract.

**Structure Alignment:**
The project structure supports the architecture:

- `src/` owns frontend UI/state only.
- `src-tauri/src/app/` owns desktop shell lifecycle.
- `src-tauri/src/tasks/` owns task domain and persistence.
- `src-tauri/migrations/` owns schema evolution.
- Tests are placed where they match component ownership.

### Requirements Coverage Validation

**Feature Coverage:**
All Phase 1 features have architectural support:

- tray access;
- compact panel;
- top input;
- `Enter` add;
- active task list;
- row-level delete;
- full edit window;
- local persistence;
- offline operation;
- explicit quit/control.

Phase 2 mobile/sync is not implemented in Phase 1, but the data model supports later sync through stable IDs, timestamps, and deletion markers.

**Functional Requirements Coverage:**
All FR groups are supported:

- FR1-FR8: task model, repository, DTOs, UI task rendering.
- FR9-FR16: Tauri tray/window modules and tray panel UI.
- FR17-FR21: full edit window and shared task command/store boundary.
- FR22-FR26: SQLite, SQLx repository, migrations, restart persistence.
- FR27-FR41: deferred to Phase 2 with sync-ready model support.
- FR42-FR45: tray menu/control, app lifecycle, status UI.

**Non-Functional Requirements Coverage:**
NFRs are architecturally addressed:

- Performance: local SQLite, no backend, no heavy frontend framework.
- Reliability: Rust-side repository, migrations, transaction-oriented writes.
- Data safety: soft delete, source-of-truth SQLite, command success before UI mutation.
- Security/privacy: no account, cloud, telemetry, or external transfer in Phase 1.
- Linux compatibility: Tauri tray validation is first implementation priority.
- Maintainability: explicit module boundaries and consistency rules.

### Implementation Readiness Validation

**Decision Completeness:**
Critical decisions are documented:

- Tauri v2 shell.
- Official Tauri Vanilla TypeScript starter.
- SQLx + SQLite persistence.
- Tauri command boundary.
- No Node ORM/sidecar.
- No backend/cloud/account in Phase 1.
- Phase 2 sync deferred.

**Structure Completeness:**
The project tree is specific enough for implementation agents. Each FR category maps to explicit files/directories.

**Pattern Completeness:**
The architecture defines naming, structure, command, error, date/time, state, delete, and loading patterns. These rules are enough to prevent major agent conflicts.

### Gap Analysis Results

**Critical Gaps:**
None.

**Important Gaps:**

- The exact MVP Linux validation baseline must still be named before implementation stories begin. Examples: KDE Plasma tray, GNOME with AppIndicator extension, or another concrete desktop environment.
- Test runner choice for TypeScript is deferred until the frontend scaffold exists.

**Nice-to-Have Gaps:**

- Packaging strategy.
- Auto-update strategy.
- Phase 2 sync protocol.
- Mobile framework.
- Recovery/trash UX.

### Validation Issues Addressed

The main risk identified in PRD validation was NFR specificity around Linux tray compatibility. Architecture addresses this by making the tray/AppIndicator proof the first implementation priority and requiring a named validation baseline before stories begin.

### Architecture Completeness Checklist

**Requirements Analysis**

- [x] Project context thoroughly analyzed
- [x] Scale and complexity assessed
- [x] Technical constraints identified
- [x] Cross-cutting concerns mapped

**Architectural Decisions**

- [x] Critical decisions documented with versions
- [x] Technology stack fully specified
- [x] Integration patterns defined
- [x] Performance considerations addressed

**Implementation Patterns**

- [x] Naming conventions established
- [x] Structure patterns defined
- [x] Communication patterns specified
- [x] Process patterns documented

**Project Structure**

- [x] Complete directory structure defined
- [x] Component boundaries established
- [x] Integration points mapped
- [x] Requirements to structure mapping complete

### Architecture Readiness Assessment

**Overall Status:** READY FOR IMPLEMENTATION

**Confidence Level:** High

**Key Strengths:**

- Clear Phase 1 scope boundary.
- First implementation priority targets the highest technical risk.
- Persistence architecture fits Tauri instead of forcing Node ORM assumptions.
- Data model is sync-ready without implementing sync early.
- Project structure gives agents concrete ownership boundaries.
- Patterns prevent common multi-agent conflicts.

**Areas for Future Enhancement:**

- Name and document the concrete Linux tray validation baseline.
- Add TypeScript test runner after scaffold.
- Define packaging once local MVP behavior is validated.
- Define sync protocol in Phase 2 architecture.
- Revisit frontend framework only if Vanilla TypeScript becomes limiting.

### Implementation Handoff

**AI Agent Guidelines:**

- Follow all architectural decisions exactly as documented.
- Use implementation patterns consistently across all components.
- Respect project structure and boundaries.
- Refer to this document for all architectural questions.
- Do not introduce backend, cloud, account, sync, Prisma, Drizzle, Kysely, or Node ORM runtime in Phase 1.
- Prove tray lifecycle before persistence and UI polish.

**First Implementation Priority:**

Scaffold the official Tauri Vanilla TypeScript app, then prove Linux tray behavior on the chosen MVP Linux validation baseline:

```bash
pnpm create tauri-app taskbar-todolist-desktop
pnpm install
pnpm tauri dev
```

The first implementation story should validate:

- tray icon appears;
- clicking tray icon opens compact panel/window;
- input can autofocus;
- closing panel does not quit the process;
- explicit quit works.
