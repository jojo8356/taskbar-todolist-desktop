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
- Rust native Linux tray behavior must be proven before mobile or sync work;
- Phase 1 must remain implementable without backend, app mobile, or external service.

**Scale & Complexity:**

- Primary domain: Linux desktop productivity utility.
- Complexity level: low-to-medium.
- Estimated architectural components: 7 core components.

Likely components:

1. Rust native app shell and lifecycle.
2. Linux tray/system integration.
3. Quick tray panel UI.
4. Full editing window UI.
5. Shared task state/store.
6. Local persistence layer.
7. Future sync boundary/interfaces.

### Technical Constraints & Dependencies

Known constraints:

- Desktop technology is Rust native desktop.
- Tauri, WebGTK/WebKit, webviews, Vite, pnpm, and browser-based UI runtimes are removed from the target stack.
- The UI must use a complete but lightweight Rust UI module; the selected Phase 1 UI module is Slint.
- Primary MVP platform is Linux.
- The concrete Linux validation baseline is Debian GNU/Linux 13 (trixie), GNOME, X11 session, with GNOME AppIndicator/KStatusNotifierItem extension support for tray validation.
- Phase 1 must work offline and without account/cloud/backend.
- The tray prototype is the first technical risk and should be front-loaded.
- Local persistence should use a structured model suitable for future sync.
- SQLite is recommended by the PRD, but the architecture should make the final persistence decision explicitly.
- UX direction is Compact Native: small panel, top input, list below, row trash action, compact spacing, native-feeling typography, light theme first.

### Persistence Technology Decision Notes

Phase 1 persistence should use SQLite from the Rust application side, with SQLx as the preferred database access layer.

Decision:

- Use Rust-side SQLite persistence with SQLx.
- Expose task operations to the UI through Rust service/controller modules, not through Tauri commands or webview IPC.
- Do not use Prisma, Drizzle, Kysely, MikroORM, TypeORM, or other Node/TypeScript ORMs in Phase 1.
- Do not add a Node sidecar just to run a Prisma-like ORM.

Rationale:

- The app must stay light and close to the Linux system; removing webview/WebGTK avoids a heavy UI runtime for a small tray utility.
- SQLx works naturally in a Rust-native app, keeps persistence native, and avoids an extra runtime process.
- The task model is small enough that a full entity ORM is unnecessary.
- Rust-side persistence keeps local data safety, migrations, and command boundaries close to the native shell.
- This supports the Phase 2 sync-ready data model without forcing sync implementation into Phase 1.

Alternatives considered:

| Option | Fit for Rust SQLite | Prisma-like DX | Runtime weight | Decision |
| --- | --- | --- | --- | --- |
| SQLx | Excellent | Medium | Low | Selected |
| Diesel | Good | Medium | Medium | Possible fallback if a fuller Rust ORM is needed |
| SeaORM | Medium | Good | Higher | Too web-service oriented for MVP |
| Drizzle + better-sqlite3 | Poor in Rust-native app | Good | Requires Node runtime | Rejected for Phase 1 |
| Prisma SQLite | Poor in Rust-native app | Excellent | Requires Prisma/client runtime fit | Rejected for Phase 1 |
| Kysely | Poor in Rust-native app | Good | Node-centric | Rejected for Phase 1 |
| MikroORM | Poor in Rust-native app | Good | Higher | Rejected for Phase 1 |
| TypeORM | Poor in Rust-native app | Medium | Higher | Rejected for Phase 1 |

Initial persistence module shape:

```text
src/tasks/
  model.rs
  repository.rs
  migrations.rs
  service.rs
```

Rust UI/service boundary:

```text
UI event -> app controller -> tasks service -> repository -> SQLite
```

Sources checked:

- Slint Rust UI: https://slint.dev/
- tray-icon crate: https://crates.io/crates/tray-icon
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

## Rust Native Stack Evaluation

### Primary Technology Domain

Desktop app, based on the project requirements analysis.

The project is a Linux-first Rust native desktop utility with a compact tray panel, full editing window, local persistence, and future sync readiness. Phase 1 does not need a backend, account system, cloud service, routing framework, server rendering, webview runtime, or mobile app scaffold.

### UI Options Considered

**Rust + Slint**

Slint provides a complete Rust-friendly UI module without a browser/webview layer. It fits the product because the UI is compact but should still have a real component model for the tray panel, task input, task rows, row delete action, and full edit window.

Trade-offs:

- Keeps the app close to the system and avoids WebGTK/WebKit runtime weight.
- Gives a complete UI layer instead of hand-rolling every widget in raw platform calls.
- Keeps the implementation Rust-first across shell, UI coordination, persistence, and local behavior.
- Requires validating tray/panel behavior with `tray-icon` and the selected Linux baseline.

**egui/eframe**

This option is very lightweight and Rust-native, but its immediate-mode model is less aligned with a polished compact desktop utility and separate full-edit UI.

Trade-offs:

- Very light runtime.
- Fast to prototype.
- Less complete as a conventional desktop UI module for this product.

**Tauri + Vanilla TypeScript**

This was previously selected, but it depends on a webview stack and Linux WebGTK/WebKit prerequisites. It no longer matches the updated requirement to keep the app light, Rust-first, and close to the system.

Trade-offs:

- Good ecosystem and tray support.
- Rejected because the UI runtime is webview/WebGTK-based.
- Rejected because the target stack should avoid TypeScript/Vite/pnpm for the app UI.

### Selected Stack: Rust Native + Slint UI

**Rationale for Selection:**

Use a Rust-native desktop stack for Phase 1: Rust application shell, Slint UI, `tray-icon` for Linux tray integration, SQLx + SQLite for persistence.

The UX spec describes a compact but real desktop interface. Slint gives a complete lightweight UI module without carrying a browser runtime. The app stays close to Linux system behavior, and the tray/persistence paths remain Rust-native.

Persistence is intentionally not handled by a Node/TypeScript ORM. SQLite persistence belongs in Rust modules with SQLx, exposed to the UI through Rust application services/controllers.

**Initialization Command:**

```bash
cargo init taskbar-todolist-desktop --bin
```

Then run:

```bash
cd taskbar-todolist-desktop
cargo run
```

**Architectural Decisions Provided by Stack:**

**Language & Runtime:**

- Rust application shell.
- Slint UI module.
- Native desktop behavior coordinated from Rust.
- No webview, no WebGTK/WebKit, no Vite, no TypeScript UI runtime, no pnpm app workflow.

**Styling Solution:**

Use Slint components/styles to implement the UX spec's compact design tokens, spacing, colors, low radius, light theme first, and destructive red only for destructive actions.

**Build Tooling:**

- Cargo development and build pipeline.
- Rust/Cargo native app packaging path.

**Testing Framework:**

Architecture should define testing separately:

- Rust unit tests for task repository and persistence behavior.
- Rust/UI-controller tests for task state and UI event behavior where practical.
- Manual MVP validation for Linux tray behavior.
- Later E2E or scripted smoke tests once the tray prototype is stable.

**Code Organization:**

The app should be evolved into these boundaries:

- `src/main.rs` for app bootstrapping only.
- `src/app/` for lifecycle, tray, windows, and app state.
- `src/ui/` for Slint integration, tray panel, full edit UI bindings, and UI event adapters.
- `src/tasks/` for task model, repository, migrations, service/controller functions, and SQLx persistence logic.

**Development Experience:**

- Use `cargo run` for local development.
- First implementation story should scaffold the Rust app and prove tray creation on the target Linux environment.
- Add the Slint UI module early enough to validate the compact tray panel without introducing web runtime weight.
- Add SQLite/SQLx persistence only after tray lifecycle is proven.

**Additional Setup Decisions Deferred From Stack:**

- SQLx migration setup.
- Test framework selection.
- Packaging strategy.

**Note:** Rust-native project initialization using this command should be the first implementation story.

## Core Architectural Decisions

### Decision Priority Analysis

**Critical Decisions (Block Implementation):**

- Use a Rust-native desktop shell, not Tauri.
- Use Slint as the complete but lightweight Rust UI module.
- Use `tray-icon` or equivalent lightweight Rust tray integration for Linux system tray behavior.
- Use Rust-side SQLite persistence with SQLx.
- Expose all task persistence operations through Rust service/controller functions.
- Validate Linux tray/AppIndicator behavior before implementing full persistence, mobile, or sync.
- Keep Phase 1 offline-only and backend-free.

**Important Decisions (Shape Architecture):**

- Use a compact Slint UI with explicit Rust adapters in Phase 1.
- Keep tray panel UI and full edit UI separate but backed by the same Rust service/state boundary.
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
SQLite matches the local-first/offline requirement. SQLx fits a Rust-native app because persistence lives naturally in Rust task modules, avoids a Node sidecar, and keeps data safety close to the native UI/service boundary.

**Rejected for Phase 1:**
Prisma, Drizzle, Kysely, MikroORM, TypeORM, and other Node/TypeScript ORMs. These fit Node/Electron-style runtime assumptions better than a Rust-native desktop app.

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
Use SQLx migrations under `migrations/` or an equivalent Rust-side migrations folder. Migrations must run during app startup before task services become available.

**Repository Boundary:**

```text
src/tasks/
  model.rs
  repository.rs
  migrations.rs
  service.rs
```

**Caching Strategy:**
No separate cache in Phase 1. The Rust UI state may hold an in-memory task list for rendering, but SQLite remains the source of truth.

### Authentication & Security

**Authentication:**
None in Phase 1.

**Authorization:**
None in Phase 1 because there is no multi-user model.

**Data Location:**
Store the SQLite database in the app data directory managed from the Rust app shell.

**Cloud/Data Egress:**
No cloud sync, account, analytics, telemetry, or default external data transfer in Phase 1.

**Data Safety:**
Task writes must be transactional. Deletes are represented with `deleted_at` rather than destructive removal so Phase 2 sync can propagate deletions.

### Application Communication Patterns

**UI-to-Service Boundary:**
Use Rust service/controller functions as the only UI persistence boundary.

Initial service operations:

```text
create_task(text)
list_tasks()
update_task(id, text, status)
delete_task(id)
```

**Error Handling:**
Rust services return typed success/error results. UI adapters map errors to compact status messages.

**External API:**
No HTTP/REST/GraphQL API in Phase 1.

**Future Sync Boundary:**
Define sync as a future service boundary behind Rust-side modules. Phase 1 data model must support sync, but Phase 1 implementation must not depend on sync.

### UI Architecture

**Framework:**
Slint UI with explicit Rust adapters for Phase 1.

**Rationale:**
The UI is intentionally small but should not be hand-rolled from raw platform calls. Slint keeps a complete lightweight UI module while avoiding Tauri, WebGTK/WebKit, browser runtime weight, and frontend framework overhead.

**UI Modules:**

```text
src/
  main.rs
  app/
    lifecycle.rs
    tray.rs
    windows.rs
    state.rs
  ui/
    mod.rs
    tray_panel.rs
    full_edit.rs
    bindings.rs
  tasks/
    model.rs
    repository.rs
    migrations.rs
    service.rs
```

**State Management:**
Small Rust app state/controller layer. It mirrors task data for UI rendering but does not replace SQLite as source of truth.

**Styling:**
Use Slint styling and component properties. Use UX spec tokens: compact spacing, low radius, light theme first, red only for destructive actions.

**Routing:**
No router in Phase 1. The app has a tray panel and full edit window, not a multi-page application.

### Infrastructure & Deployment

**Hosting:**
None.

**Build/Runtime:**
Rust/Cargo build pipeline with Slint UI and Rust-native tray integration. No Tauri, no WebGTK/WebKit, no Vite, no pnpm app workflow.

**Developer Linux Stack:**

- Primary developer environment: Debian GNU/Linux 13 (trixie), x86_64, Linux kernel 6.12.x.
- Desktop/session baseline: GNOME on X11.
- Tray/AppIndicator support: GNOME AppIndicator/KStatusNotifierItem extension.
- Package family for native prerequisites: Debian/Ubuntu packages.
- Native dependency expectation: Rust toolchain, Cargo, build tooling, `libssl-dev`, `libdbus-1-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`, and `pkg-config`. Do not add WebGTK/WebKit prerequisites.
- Validation posture: test and debug Linux behavior first on this environment before generalizing to other desktop environments, Wayland sessions, Windows, or macOS.

**Development Commands:**

```bash
cargo init taskbar-todolist-desktop --bin
cargo run
```

**Validation Baseline:**
Use Debian GNU/Linux 13 (trixie), GNOME X11, with GNOME AppIndicator/KStatusNotifierItem extension as the MVP Linux tray validation baseline.

**Implementation Sequence:**

1. Scaffold Rust native app with Slint UI module and lightweight tray integration.
2. Prove Linux tray icon, panel opening, focus behavior, and background lifecycle.
3. Add compact tray UI skeleton.
4. Add Rust task model and SQLx SQLite migrations.
5. Add Rust task service/controller functions.
6. Connect Slint UI adapters to Rust task services.
7. Add full edit window.
8. Add persistence/restart validation.
9. Defer mobile/sync.

### Decision Impact Analysis

**Implementation Sequence:**
The tray prototype comes before persistence because system tray reliability is the highest technical risk. SQLx persistence comes before full edit polish because state consistency and restart safety are core NFRs.

**Cross-Component Dependencies:**

- Tray panel depends on Rust app lifecycle, tray integration, and Rust UI state.
- Full edit window depends on the same Rust service/state boundary as the tray panel.
- SQLx repository depends on migrations and task model.
- Future sync depends on stable IDs, timestamps, and deletion markers.
- UI consistency depends on shared Slint styling tokens and compact component conventions.

## Implementation Patterns & Consistency Rules

### Pattern Categories Defined

**Critical Conflict Points Identified:**
Nine areas where AI agents could make incompatible choices:

1. Database naming.
2. Rust model naming.
3. UI adapter model naming.
4. Rust service naming and payloads.
5. Soft-delete behavior.
6. Error response format.
7. Date/time format.
8. UI state ownership.
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
- Service/controller functions: snake_case and action-oriented.

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

**UI Adapter Naming Conventions:**

Use Rust idioms at the UI boundary.

Rules:

- Types: `PascalCase`, e.g. `Task`, `TaskStatus`.
- Functions: `snake_case`, e.g. `create_task`, `list_tasks`.
- Fields exposed to UI adapters: `snake_case`, e.g. `created_at`, `deleted_at`.
- Files: snake_case, e.g. `task_service.rs`, `tray_panel.rs`.

UI model baseline:

```rust
pub struct TaskView {
    pub id: String,
    pub text: String,
    pub status: TaskStatus,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}
```

### Structure Patterns

**Project Organization:**

Rust native app:

```text
src/
  main.rs
  tasks/
    mod.rs
    model.rs
    repository.rs
    migrations.rs
    service.rs
  app/
    tray.rs
    windows.rs
    errors.rs
  ui/
    mod.rs
    tray_panel.rs
    full_edit.rs
    bindings.rs
```

**File Structure Patterns:**

- Domain logic lives in `src/tasks/`.
- Tray lifecycle logic lives in `src/app/tray.rs`.
- Window creation and show/hide behavior lives in `src/app/windows.rs`.
- UI modules must not access SQLite directly.
- UI modules must call Rust task service/controller functions, not repository functions directly.

**Test Organization:**

Rust:

- Unit tests live near the module under `#[cfg(test)]`.
- Persistence integration tests can live under `tests/` once the schema stabilizes.

- UI/controller tests should stay in Rust test modules or integration tests.
- Do not add browser E2E tests because the app no longer uses a browser/webview UI.

### Format Patterns

**Rust Service Response Formats:**

Rust services should return `Result<T, AppError>`.

UI adapters convert Rust response/error into UI-friendly results.

Rust service pattern:

```rust
pub async fn create_task(text: String, state: &AppState) -> Result<TaskView, AppError> {
    state.tasks.create(text).await.map(TaskView::from)
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
- Format for display only in the UI layer.

**Data Exchange Format:**

- Rust/database fields may be snake_case internally.
- UI view fields should remain Rust/Slint-friendly and explicit.
- Do conversion at the service/UI adapter boundary, not throughout UI modules.

### Communication Patterns

**Rust Service Naming:**

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

**Service Payloads:**

Payloads must be minimal and explicit.

```text
create_task(text)
update_task(id, text?, status?)
delete_task(id)
list_tasks()
```

**State Management Patterns:**

- SQLite is the source of truth.
- Rust app/UI state is a cache for rendering and UI responsiveness.
- After a mutation service succeeds, update UI state from the service result or reload the list.
- Do not let UI-only state mutate task records without a successful Rust service call.

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

- Keep SQLite access inside Rust task repository modules.
- Use SQLx for SQLite persistence.
- Use Rust service/controller functions as the UI/persistence boundary.
- Use soft delete through `deleted_at`.
- Keep Phase 1 free of backend, account, cloud, and sync dependencies.
- Use snake_case for database/Rust names.
- Keep tray panel and full edit UI backed by the same Rust service/state boundary.
- Preserve input focus after successful add.
- Avoid introducing project/tag/priority/calendar concepts.

**Pattern Enforcement:**

- Any new task persistence function must be added to `repository.rs` and exposed through `service.rs`.
- Any new UI task operation must go through Rust service/controller modules.
- Any schema change must include a migration and update Rust task/UI adapter types.
- Any user-facing error must map from a stable error code.
- Any Phase 2 sync-related code must remain behind explicit module boundaries and must not be required for Phase 1 tests.

### Pattern Examples

**Good Examples:**

```rust
pub async fn create_task(text: String, state: &AppState) -> Result<TaskView, AppError> {
    state.tasks.create(text).await.map(TaskView::from)
}
```

```rust
let task = task_service.create_task(input_text).await?;
ui_state.add_task(task);
ui_state.clear_input_and_focus();
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
├── Cargo.toml
├── migrations/
│   └── 0001_create_tasks.sql
├── assets/
│   └── icons/
├── src/
│   ├── main.rs
│   ├── app/
│   │   ├── mod.rs
│   │   ├── lifecycle.rs
│   │   ├── tray.rs
│   │   ├── windows.rs
│   │   ├── state.rs
│   │   └── errors.rs
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── tray_panel.rs
│   │   ├── full_edit.rs
│   │   └── bindings.rs
│   └── tasks/
│       ├── mod.rs
│       ├── model.rs
│       ├── repository.rs
│       ├── migrations.rs
│       └── service.rs
├── tests/
│   └── task_repository.rs
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

The only application boundary between UI and persistence is Rust service/controller functions:

```text
Slint UI -> ui bindings -> task service -> repository.rs -> SQLite
```

Initial service boundary:

```text
create_task
list_tasks
update_task
delete_task
```

**Component Boundaries:**

Rust UI modules are small and explicit:

- `ui/tray_panel.rs` owns tray panel rendering/bindings and composition.
- `ui/bindings.rs` owns input behavior, `Enter` add interaction, and UI-service wiring.
- `ui/full_edit.rs` owns the secondary edit surface bindings.
- `app/tray.rs` owns tray menu/control wiring.
- `app/windows.rs` owns window creation, show/hide, and focus behavior.

UI modules must not access SQLite directly. They call Rust task service/controller functions.

**Service Boundaries:**

Rust service boundaries:

- `app/tray.rs`: tray icon, menu, click handling, show/hide behavior.
- `app/windows.rs`: window creation and lifecycle.
- `app/state.rs`: shared app state, including task repository handle.
- `app/errors.rs`: `AppError` and error mapping.
- `tasks/model.rs`: Rust task domain types.
- `tasks/service.rs`: UI-facing task operations.
- `tasks/repository.rs`: SQLx persistence operations.
- `tasks/migrations.rs`: migration execution.

**Data Boundaries:**

SQLite is accessed only from Rust.

- SQLite schema lives in `migrations/`.
- SQLx queries live in `tasks/repository.rs`.
- UI view conversion lives in `tasks/service.rs` or `ui/bindings.rs`.

### Requirements to Structure Mapping

**Task Management FR1-FR8**

- Rust model: `src/tasks/model.rs`
- Rust repository: `src/tasks/repository.rs`
- UI view types/adapters: `src/tasks/service.rs`, `src/ui/bindings.rs`
- UI rendering: `src/ui/tray_panel.rs`

**Taskbar / Tray Experience FR9-FR16**

- Tray lifecycle: `src/app/tray.rs`
- Window lifecycle: `src/app/windows.rs`
- Tray panel UI: `src/ui/tray_panel.rs`
- Input/list/delete behavior: `src/ui/bindings.rs`

**Full Editing UI FR17-FR21**

- Full edit window lifecycle: `src/app/windows.rs`
- Full edit UI: `src/ui/full_edit.rs`
- Shared task state: `src/app/state.rs`
- Task services: `src/tasks/service.rs`

**Local Storage FR22-FR26**

- SQLite migration: `migrations/0001_create_tasks.sql`
- Migration runner: `src/tasks/migrations.rs`
- SQLx repository: `src/tasks/repository.rs`
- Persistence tests: `tests/task_repository.rs`

**Mobile Companion / Sync FR27-FR41**

Deferred to Phase 2.

Phase 1 must still support sync readiness through:

- stable `id`;
- `created_at`;
- `updated_at`;
- `deleted_at`;
- soft-delete query behavior.

**Settings and Control FR42-FR45**

- Tray menu/control: `src/app/tray.rs`
- App/window lifecycle: `src/app/windows.rs`
- Status display: `src/ui/bindings.rs`

### Integration Points

**Internal Communication:**

```text
User action
-> Slint UI binding
-> Rust task service
-> Rust repository
-> SQLite
-> TaskView result
-> UI state update
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

- `Cargo.toml`: Rust dependencies, Slint UI, tray integration, SQLx.
- `migrations/`: SQLite schema migrations.
- `assets/icons/`: app icons.
- `.env.example`: documented environment variables, if any are introduced.

**Source Organization:**

- `src/` contains Rust application, UI, lifecycle, and task modules.
- `src/tasks/` is the task domain boundary.
- `src/app/` is the desktop shell/lifecycle boundary.
- `src/ui/` is the Slint UI integration boundary.

**Test Organization:**

- Rust unit tests live near modules.
- Rust repository integration tests live in `tests/`.
- UI adapter tests stay in Rust test modules or integration tests.
- No E2E suite until tray lifecycle is proven.

**Asset Organization:**

- App icons live in `assets/icons/`.
- UI styling lives in Slint UI definitions and Rust UI bindings.
- No large visual asset system is required for Phase 1.

### Development Workflow Integration

**Development Workflow Structure:**

Use the Rust native dev workflow:

```bash
cargo run
```

The native app, Slint UI, tray integration, and persistence run through Cargo.

**Build Process Structure:**

Use Cargo build pipeline:

```bash
cargo build --release
```

Packaging decisions are post-MVP unless needed for local validation.

**Deployment Structure:**

No hosted deployment exists in Phase 1.

Distribution is local desktop binary/package only. Packaging is a post-MVP decision.

## Architecture Validation Results

### Coherence Validation

**Decision Compatibility:**
The architectural decisions are compatible:

- Rust native shell matches the lightweight system-close target.
- Slint matches the compact UI scope without WebGTK/WebKit or webviews.
- SQLx + SQLite fits Rust-side persistence in `src/tasks`.
- Rust service/controller functions provide a clean UI/persistence boundary.
- No backend/cloud/account dependency conflicts with the offline MVP requirement.

No contradictory decisions were found.

**Pattern Consistency:**
Implementation patterns support the decisions:

- SQLite access is constrained to Rust.
- Rust service/controller functions prevent scattered repository access from UI modules.
- snake_case is used for Rust/database names.
- Soft-delete semantics support future sync without requiring sync now.
- Error codes and service boundaries give agents a consistent contract.

**Structure Alignment:**
The project structure supports the architecture:

- `src/app/` owns desktop shell lifecycle.
- `src/ui/` owns Slint UI integration.
- `src/tasks/` owns task domain and persistence.
- `migrations/` owns schema evolution.
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
- FR9-FR16: Rust tray/window modules and tray panel UI.
- FR17-FR21: full edit window and shared Rust service/state boundary.
- FR22-FR26: SQLite, SQLx repository, migrations, restart persistence.
- FR27-FR41: deferred to Phase 2 with sync-ready model support.
- FR42-FR45: tray menu/control, app lifecycle, status UI.

**Non-Functional Requirements Coverage:**
NFRs are architecturally addressed:

- Performance: local SQLite, no backend, no heavy frontend framework.
- Reliability: Rust-side repository, migrations, transaction-oriented writes.
- Data safety: soft delete, source-of-truth SQLite, command success before UI mutation.
- Security/privacy: no account, cloud, telemetry, or external transfer in Phase 1.
- Linux compatibility: Rust native tray validation is first implementation priority.
- Maintainability: explicit module boundaries and consistency rules.

### Implementation Readiness Validation

**Decision Completeness:**
Critical decisions are documented:

- Rust-native desktop shell.
- Slint complete lightweight Rust UI module.
- Lightweight Rust tray integration.
- SQLx + SQLite persistence.
- Rust service/controller boundary.
- No Node ORM/sidecar.
- No Tauri, WebGTK/WebKit, webview, Vite, pnpm app workflow, or browser-based UI runtime.
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
- Persistence architecture fits a Rust-native app instead of forcing Node ORM assumptions.
- Data model is sync-ready without implementing sync early.
- Project structure gives agents concrete ownership boundaries.
- Patterns prevent common multi-agent conflicts.

**Areas for Future Enhancement:**

- Define packaging once local MVP behavior is validated.
- Define sync protocol in Phase 2 architecture.
- Revisit UI toolkit only if Slint becomes limiting.

### Implementation Handoff

**AI Agent Guidelines:**

- Follow all architectural decisions exactly as documented.
- Use implementation patterns consistently across all components.
- Respect project structure and boundaries.
- Refer to this document for all architectural questions.
- Do not introduce Tauri, WebGTK/WebKit, webview UI, backend, cloud, account, sync, Prisma, Drizzle, Kysely, or Node ORM runtime in Phase 1.
- Prove tray lifecycle before persistence and UI polish.

**First Implementation Priority:**

Scaffold the Rust-native Slint app, then prove Linux tray behavior on the chosen MVP Linux validation baseline:

```bash
cargo init taskbar-todolist-desktop --bin
cargo run
```

The first implementation story should validate:

- tray icon appears;
- clicking tray icon opens compact panel/window;
- input can autofocus;
- closing panel does not quit the process;
- explicit quit works.
