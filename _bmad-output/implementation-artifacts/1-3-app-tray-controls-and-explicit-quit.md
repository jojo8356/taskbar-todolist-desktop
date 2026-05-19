# Story 1.3: App Tray Controls and Explicit Quit

Status: done

<!-- Note: Validation is optional. Run validate-create-story for quality check before dev-story. -->

## Story

As a desktop user,
I want minimal tray controls and an explicit quit action,
so that I can understand and control the app lifecycle.

## Acceptance Criteria

1. Given the app is running in the tray, when the user opens available tray controls, then the app exposes minimal control actions needed for Phase 1.
2. Given the app is running in the background, when the user chooses explicit quit, then the app exits completely and the tray icon disappears.
3. Given the app is running normally, when the tray state is displayed, then the user can identify the general app state from the tray or compact status UI.

## Tasks / Subtasks

- [x] Add minimal tray menu controls in Rust. (AC: 1, 2)
  - [x] Keep tray/system access in Rust, not TypeScript.
  - [x] Add tray menu items for opening the panel, hiding the panel, and explicit quit.
  - [x] Keep the Linux-safe menu path for opening the panel.
- [x] Add explicit quit lifecycle behavior. (AC: 2)
  - [x] Add a Rust `quit_app` command.
  - [x] Wire tray quit menu item to exit the app process.
  - [x] Wire any panel quit affordance through the Rust command only.
- [x] Add compact app state visibility. (AC: 3)
  - [x] Add minimal tray/status text in the compact panel that identifies local desktop status.
  - [x] Use Lucide icons for visible panel controls only, with targeted imports.
  - [x] Do not add sync mode selection, settings screens, persistence, backend, mobile, or task CRUD in this story.
- [x] Perform story validation. (AC: 1-3)
  - [x] Run `cargo fmt --check`.
  - [x] Confirm `pnpm build` passes.
  - [x] Attempt `pnpm tauri dev` and record the current actionable system blocker if native validation cannot complete.
  - [x] Confirm direct manifests do not include React, Vue, Svelte, Tailwind, SQLite, SQLx, sync, backend, mobile, cloud, Prisma, Drizzle, Kysely, MikroORM, TypeORM, or Node ORM sidecar.

### Review Findings

- [x] [Review][Patch] Move raw lifecycle invokes out of UI modules into a command wrapper [src/ui/tray-panel.ts:1]

## Dev Notes

### Previous Story Intelligence

- Story 1.2 added Rust-owned tray lifecycle in `src-tauri/src/app/tray.rs` and Rust-owned panel show/hide/focus behavior in `src-tauri/src/app/windows.rs`.
- Story 1.2 added a compact Vanilla TypeScript panel shell in `src/ui/tray-panel.ts` and autofocus input in `src/ui/task-input.ts`.
- User explicitly confirmed the correct split: Rust owns taskbar/system tray access; TypeScript owns only the in-panel UI.
- Lucide is already installed as the vanilla `lucide` package and should remain UI-only.
- Rustup is installed locally with `rustc 1.95.0`, `cargo 1.95.0`, and `rustfmt 1.9.0-stable`.
- Current `pnpm tauri dev` validation reaches native compilation and stops on missing `dbus-1.pc`; install `libdbus-1-dev` and `pkg-config` on Debian/Ubuntu-family systems.

### Architecture Guardrails

- Tray menu/control and app lifecycle belong in `src-tauri/src/app/tray.rs` and `src-tauri/src/app/windows.rs`. [Source: _bmad-output/planning-artifacts/architecture.md#Settings-and-Control-FR42-FR45]
- Explicit quit is part of the tray lifecycle proof before persistence/mobile/sync. [Source: _bmad-output/planning-artifacts/architecture.md#Gap-Analysis-Results]
- Keep Phase 1 backend-free, cloud-free, account-free, mobile-free, sync-free, and ORM-free. [Source: _bmad-output/planning-artifacts/architecture.md#Core-Architectural-Decisions]
- Do not add browser E2E tests until tray behavior is proven manually. [Source: _bmad-output/planning-artifacts/architecture.md#Test-Organization]

### UX Requirements

- Minimal controls only: open panel, hide panel, explicit quit, and a compact status signal.
- Red/destructive styling is appropriate only for explicit quit. [Source: _bmad-output/planning-artifacts/epics.md#UX-DR17]
- Tray panel text scale should stay compact, with secondary/status text around `12px/16px`. [Source: _bmad-output/planning-artifacts/epics.md#UX-DR20]

### Expected Files to Touch

- `src-tauri/src/app/tray.rs`
- `src-tauri/src/app/windows.rs`
- `src-tauri/src/lib.rs`
- `src/state/app-commands.ts`
- `src/ui/tray-panel.ts`
- `src/styles.css`
- `_bmad-output/implementation-artifacts/sprint-status.yaml`

### References

- [Source: _bmad-output/planning-artifacts/epics.md#Story-1-3-App-Tray-Controls-and-Explicit-Quit]
- [Source: _bmad-output/planning-artifacts/architecture.md#Settings-and-Control-FR42-FR45]
- [Source: _bmad-output/planning-artifacts/prd.md#Functional-Requirements]

## Dev Agent Record

### Agent Model Used

GPT-5

### Debug Log References

- `cargo fmt --manifest-path src-tauri/Cargo.toml`
- `cargo fmt --manifest-path src-tauri/Cargo.toml -- --check`
- `pnpm build`
- `pnpm tauri dev`

### Completion Notes List

- Ultimate context engine analysis completed - comprehensive developer guide created.
- Added Rust tray menu controls: `Open Taskbar Todolist`, `Hide Panel`, and `Quit`.
- Added Rust `quit_app` command and wired panel quit affordance through Tauri invoke.
- Added compact status strip in the tray panel identifying local desktop/tray process state.
- Kept Lucide usage UI-only with targeted `ListTodo`, `X`, and `Power` imports.
- `cargo fmt --check` passes.
- `pnpm build` passes.
- `pnpm tauri dev` reaches native compilation and currently stops on missing system package support for `dbus-1.pc`; install `libdbus-1-dev` and `pkg-config` on Debian/Ubuntu-family systems.

### File List

- `_bmad-output/implementation-artifacts/1-3-app-tray-controls-and-explicit-quit.md`
- `_bmad-output/implementation-artifacts/sprint-status.yaml`
- `src-tauri/src/app/tray.rs`
- `src-tauri/src/lib.rs`
- `src/ui/tray-panel.ts`
- `src/styles.css`

### Change Log

- 2026-05-19: Implemented Story 1.3 tray controls, explicit quit, compact status UI, and validation notes.
- 2026-05-19: Addressed code review finding: panel lifecycle actions now call command wrappers instead of raw `invoke(...)`.
