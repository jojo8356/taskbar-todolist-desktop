# Story 1.2: Tray Icon Opens Compact Panel Without Quitting App

Status: done

<!-- Note: Validation is optional. Run validate-create-story for quality check before dev-story. -->

## Story

As a desktop user,
I want to open Taskbar Todolist from the Linux tray,
so that I can access tasks without opening a full application window.

## Acceptance Criteria

1. Given the app is running on the MVP Linux validation baseline, when the app starts, then a tray/system indicator icon is visible.
2. Given the tray icon is visible, when the user clicks the tray icon, then a compact panel or panel-like window opens.
3. Given the compact panel is open, when the user closes or hides the panel, then the app process remains running in the background and the tray icon remains available.
4. Given the compact panel opens, when the input is rendered, then the input can receive autofocus.

## Tasks / Subtasks

- [x] Add Tauri tray lifecycle and compact panel window plumbing. (AC: 1, 2, 3)
  - [x] Add `tray-icon` feature to the Tauri dependency only; do not add persistence, sync, backend, mobile, or ORM dependencies.
  - [x] Create `src-tauri/src/app/tray.rs` for tray/menu setup and tray-triggered panel opening.
  - [x] Create `src-tauri/src/app/windows.rs` for panel show/hide/focus behavior.
  - [x] Keep the process alive when the compact panel is hidden or closed.
- [x] Replace starter UI with a compact tray panel shell. (AC: 2, 4)
  - [x] Replace starter greeting UI with a tray-panel root sized around 320px wide and panel-like.
  - [x] Add a top task input that autofocuses on render.
  - [x] Use Lucide icons via the vanilla `lucide` package for visible UI icons; import only the icons used.
  - [x] Do not implement real task persistence, add/delete behavior, or full edit UI in this story.
- [x] Add local commands needed by the tray panel lifecycle. (AC: 2, 3, 4)
  - [x] Expose a `show_tray_panel` command that can show/focus the compact panel.
  - [x] Expose a `hide_tray_panel` command that hides the panel without quitting the app.
  - [x] Wire a close/hide UI affordance to `hide_tray_panel`.
- [x] Perform story validation. (AC: 1-4)
  - [x] Confirm `pnpm build` passes.
  - [x] Attempt `pnpm tauri dev` and record whether validation reaches the known Rust toolchain blocker from Story 1.1.
  - [x] Confirm manifest dependencies do not include React, Vue, Svelte, Tailwind, SQLite, SQLx, sync, backend, mobile, cloud, Prisma, Drizzle, Kysely, MikroORM, TypeORM, or Node ORM sidecar.

### Review Findings

- [x] [Review][Patch] Fail loudly instead of creating a potentially invisible tray without an icon [src-tauri/src/app/tray.rs:16]
- [x] [Review][Patch] Move raw lifecycle invokes out of UI modules into a command wrapper [src/ui/tray-panel.ts:1]

## Dev Notes

### Previous Story Intelligence

- Story 1.1 scaffolded the official Tauri v2 Vanilla TypeScript starter at the repository root.
- `pnpm install` and `pnpm build` passed.
- `pnpm tauri dev` starts Vite on `http://localhost:1420/` and reaches Cargo, then stops because local `rustc 1.85.0` is below dependency requirements up to `rustc 1.88.0`.
- Linux validation baseline selected in `README.md`: Debian GNU/Linux 13 (trixie), GNOME, X11, GNOME AppIndicator/KStatusNotifierItem extension.

### Architecture Guardrails

- Tray lifecycle logic belongs in `src-tauri/src/app/tray.rs`; window creation and show/hide behavior belongs in `src-tauri/src/app/windows.rs`. [Source: _bmad-output/planning-artifacts/architecture.md#Service-Boundaries]
- Tray panel UI belongs in `src/ui/tray-panel.ts`; input behavior belongs in `src/ui/task-input.ts`. [Source: _bmad-output/planning-artifacts/architecture.md#Component-Boundaries]
- Keep Phase 1 Vanilla TypeScript and Tauri v2. Do not add React, Vue, Svelte, Tailwind, backend/cloud/sync, mobile runtime, or Node ORM dependencies. [Source: _bmad-output/planning-artifacts/architecture.md#Core-Architectural-Decisions]
- Do not add SQLite/SQLx persistence in this story; persistence starts after tray lifecycle is proven. [Source: _bmad-output/planning-artifacts/architecture.md#Development-Sequence-Decision]
- Do not add browser E2E tests until tray behavior is proven manually. [Source: _bmad-output/planning-artifacts/architecture.md#Test-Organization]

### Linux Tray Constraints

- Official Tauri docs require the `tray-icon` feature for Rust tray support.
- Official Tauri docs state Linux tray click events are unsupported even when the icon is shown; the icon can still show a context menu on right click. Therefore this story must provide an `Open Taskbar Todolist` tray menu item as the reliable Linux validation path. Treat direct click-to-open as best effort only if the platform emits events.
- Story 1.3 owns final tray controls and explicit quit. This story may add a minimal Open menu item only to validate panel opening.

### UX Requirements

- Compact tray panel should be around `320px` wide with an input fixed at the top and autofocus on open. [Source: _bmad-output/planning-artifacts/ux-design-specification.md#UX-Design-Specification-taskbar-todolist-desktop]
- This story should create only the shell: no real task list, no delete behavior, no full edit surface, no dashboard concepts.
- Use Lucide for UI icons. For Vanilla TypeScript, use the `lucide` package and import only the icons needed, e.g. `createIcons`, `ListTodo`, `X`.

### Expected Files to Touch

- `package.json`
- `pnpm-lock.yaml`
- `src/main.ts`
- `src/styles.css`
- `src/ui/tray-panel.ts`
- `src/ui/task-input.ts`
- `src-tauri/Cargo.toml`
- `src-tauri/src/lib.rs`
- `src-tauri/src/app/mod.rs`
- `src-tauri/src/app/tray.rs`
- `src-tauri/src/app/windows.rs`
- `src-tauri/tauri.conf.json`
- `src-tauri/capabilities/default.json`
- `README.md`

### References

- [Source: _bmad-output/planning-artifacts/epics.md#Story-1-2-Tray-Icon-Opens-Compact-Panel-Without-Quitting-App]
- [Source: _bmad-output/planning-artifacts/architecture.md#Requirements-to-Structure-Mapping]
- [Source: _bmad-output/planning-artifacts/architecture.md#Service-Boundaries]
- [Source: _bmad-output/planning-artifacts/ux-design-specification.md#UX-Design-Specification-taskbar-todolist-desktop]
- [Official Tauri System Tray](https://v2.tauri.app/learn/system-tray/)
- [Lucide Vanilla JavaScript package](https://www.npmjs.com/package/lucide)

## Dev Agent Record

### Agent Model Used

GPT-5

### Debug Log References

- `pnpm add lucide`
- `cargo fmt --manifest-path src-tauri/Cargo.toml`
- `pnpm build`
- `pnpm tauri dev`

### Completion Notes List

- Ultimate context engine analysis completed - comprehensive developer guide created.
- Implemented Rust-owned tray lifecycle in `src-tauri/src/app/tray.rs`; tray activation opens the compact panel.
- Replaced the deprecated Tauri/libayatana appindicator tray backend with a direct Linux StatusNotifierItem implementation over D-Bus via `zbus`.
- Implemented Rust-owned panel show/hide/focus lifecycle in `src-tauri/src/app/windows.rs`; close requests hide the panel instead of quitting the app.
- Replaced the starter greeting UI with a compact Vanilla TypeScript tray panel shell and autofocus input.
- Added Lucide UI icons through the vanilla `lucide` package using targeted imports only.
- Installed rustup user toolchain during validation; `rustc 1.95.0`, `cargo 1.95.0`, and `rustfmt 1.9.0-stable` are now available.
- `pnpm build` passes.
- `cargo tree --manifest-path src-tauri/Cargo.toml -i libappindicator` prints no active dependency.
- `timeout 20s pnpm tauri dev` starts the app without the previous `libayatana-appindicator` deprecation warning.
- `pnpm tauri dev` now starts successfully with local vendored native pkg-config support for the required Linux build dependencies.

### File List

- `_bmad-output/implementation-artifacts/1-2-tray-icon-opens-compact-panel-without-quitting-app.md`
- `_bmad-output/implementation-artifacts/sprint-status.yaml`
- `README.md`
- `index.html`
- `package.json`
- `pnpm-lock.yaml`
- `src/main.ts`
- `src/styles.css`
- `src/ui/task-input.ts`
- `src/ui/tray-panel.ts`
- `src/state/app-commands.ts`
- `src-tauri/Cargo.toml`
- `src-tauri/src/app/mod.rs`
- `src-tauri/src/app/tray.rs`
- `src-tauri/src/app/windows.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/tauri.conf.json`

### Change Log

- 2026-05-19: Implemented Story 1.2 tray lifecycle, compact panel shell, Lucide UI icons, and validation updates.
- 2026-05-19: Addressed code review findings: tray setup now fails explicitly without a default icon, and UI lifecycle commands route through `src/state/app-commands.ts`.
- 2026-05-22: Migrated Linux tray implementation from the deprecated libayatana appindicator path to direct StatusNotifierItem registration with `zbus`.
