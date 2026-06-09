# Story 1.1: Scaffold Rust Native App and Linux Validation Baseline

Status: done

> Supersedes the previous Tauri/Vanilla TypeScript scaffold decision. The active stack is Rust native + Slint UI + lightweight Rust tray integration, with no Tauri, WebGTK/WebKit, Vite, pnpm, or webview UI runtime.

<!-- Note: Validation is optional. Run validate-create-story for quality check before dev-story. -->

## Story

As a desktop user,
I want the app scaffolded with the selected Rust-native Slint stack and a named Linux validation baseline,
so that implementation starts from the agreed architecture and proves the target environment first.

## Acceptance Criteria

1. Given the repository is ready for implementation, when the app is scaffolded, then it uses a Rust-native Cargo application with Slint UI and can run through `cargo run`.
2. Given the starter has been scaffolded, when dependencies are installed, then dependency installation completes successfully and the repository documents the local development command needed to run the Rust native app.
3. Given the architecture requires a Linux tray validation baseline, when the scaffold story is completed, then the chosen MVP Linux desktop environment and tray/AppIndicator setup are documented in the repository and the validation baseline is referenced by later tray lifecycle tests.
4. Given Phase 1 must remain desktop-local and lightweight, when dependencies are added, then no Tauri, WebGTK/WebKit, webview UI runtime, Vite, pnpm app workflow, backend, cloud service, account provider, mobile framework, sync runtime, Prisma, Drizzle, Kysely, MikroORM, TypeORM, or Node ORM sidecar is introduced.

## Tasks / Subtasks

- [x] Scaffold the app with a Rust-native Cargo layout and Slint UI. (AC: 1)
  - [x] Use the project root as the final app root; avoid creating a nested app directory that leaves the repo root as a wrapper.
  - [x] Add Slint as the complete lightweight Rust UI module.
  - [x] Add lightweight Rust tray integration as optional `tray` feature for Linux tray validation.
  - [x] Preserve existing BMad artifacts, docs, and repo history while replacing obsolete Tauri/web scaffold files as needed.
- [x] Install and verify baseline dependencies. (AC: 2, 4)
  - [x] Run dependency resolution/build with Cargo.
  - [x] Verify `cargo check` succeeds; `cargo run` is documented as the local app command.
  - [x] Do not add Tauri, WebGTK/WebKit, webview UI runtime, React, Vue, Svelte, Tailwind, Vite, pnpm app workflow, sync, backend, mobile, cloud, account, or Node ORM dependencies in this story.
- [x] Document the local development command and Linux validation baseline. (AC: 2, 3)
  - [x] Update the root README so it reflects the active BMad decisions: Linux-first, Rust native, Slint UI, Phase 1 desktop-local, no mandatory backend/cloud/sync, no Tauri/WebGTK/WebKit.
  - [x] Add a concise validation baseline section naming Debian GNU/Linux 13 (trixie), GNOME X11, and GNOME AppIndicator/KStatusNotifierItem extension as the MVP tray/AppIndicator validation stack.
  - [x] Document required Linux system packages at least for Debian/Ubuntu-family environments if that matches the chosen baseline.
- [x] Remove or rewrite obsolete repo guidance. (AC: 3, 4)
  - [x] Remove stale Tauri, TypeScript frontend, Vite, pnpm, WebGTK/WebKit, `src-tauri`, and webview guidance from active scaffold docs.
  - [x] Ensure docs do not claim Phase 1 uses a backend, sync runtime, mobile framework, or browser UI runtime.
- [x] Perform story validation. (AC: 1-4)
  - [x] Confirm the scaffolded app has Rust-native `src/`, `Cargo.toml`, Slint UI integration, and no Tauri/Vite/pnpm app files.
  - [x] Confirm `cargo run` is the documented local dev command.
  - [x] Confirm `Cargo.toml` does not include forbidden Phase 1 dependencies listed in AC4.

### Review Findings

- [x] [Review][Patch] Restore repo ignore protections removed by the scaffold generator [.gitignore:1]

## Dev Notes

### Current Repository State

- The repository is still pre-scaffold. Current source files are only root `README.md`, `src/README.md`, BMad planning artifacts, and basic repo metadata.
- Root `README.md` and `src/README.md` contain stale pre-BMad concepts: backend sync, `src/tray`, `src/storage`, and `src/sync`. Treat these as obsolete and update them during this story.
- No previous implementation story exists. There are no established code patterns to preserve beyond BMad planning artifacts and repo history.

### Architecture Guardrails

- Use a Rust-native desktop shell, Slint UI, and lightweight Rust tray integration as the implementation foundation. [Source: _bmad-output/planning-artifacts/architecture.md#Selected-Stack-Rust-Native--Slint-UI]
- Do not use Tauri, WebGTK/WebKit, webviews, Vite, pnpm, or a browser-based UI runtime. [Source: _bmad-output/planning-artifacts/architecture.md#Technical-Constraints--Dependencies]
- Keep Phase 1 offline-only, backend-free, cloud-free, account-free, mobile-free, and sync-free. [Source: _bmad-output/planning-artifacts/architecture.md#Core-Architectural-Decisions]
- Do not add Prisma, Drizzle, Kysely, MikroORM, TypeORM, or a Node ORM sidecar. Persistence will be Rust-side SQLite with SQLx in later Epic 3 stories, not in this scaffold story. [Source: _bmad-output/planning-artifacts/architecture.md#Persistence-Technology-Decision-Notes]
- Do not add Tailwind CSS or frontend web styling tooling in this story. Slint owns the UI styling layer. [Source: _bmad-output/planning-artifacts/architecture.md#UI-Architecture]
- First technical risk is Linux tray/AppIndicator feasibility; use Debian GNU/Linux 13 (trixie), GNOME X11, with GNOME AppIndicator/KStatusNotifierItem extension as the validation baseline so Story 1.2 can prove tray lifecycle against it. [Source: _bmad-output/planning-artifacts/architecture.md#Infrastructure--Deployment]

### Expected Project Shape After Scaffold

The Rust-native scaffold should start close to the long-term architecture:

```text
Cargo.toml
migrations/
assets/
  icons/
src/
  main.rs
  app/
  ui/
  tasks/
tests/
```

Do not create a Tauri `src-tauri/` tree. Stories 1.2, 1.3, and Epic 3 will add tray lifecycle and persistence modules under the Rust-native structure.

### Linux Validation Baseline Requirements

The baseline must be concrete enough for later test reproduction:

- Linux distribution and version: Debian GNU/Linux 13 (trixie).
- Desktop environment and session type: GNOME X11.
- Tray/AppIndicator support mechanism: GNOME AppIndicator/KStatusNotifierItem extension.
- Required system packages installed or expected, using Debian/Ubuntu package names.
- Manual validation command, expected outcome, and known limitations.

Do not write "Linux" as the baseline by itself; that is too vague for NFR17.

### Technical Information

- Use `cargo init taskbar-todolist-desktop --bin` or equivalent root-level Cargo setup.
- Add Slint as the complete lightweight Rust UI module.
- Add lightweight Rust tray integration, for example `tray-icon`, for Linux tray validation.
- Debian/Ubuntu native package notes should not include WebGTK/WebKit. Expected native packages are Rust/Cargo, build tooling, `libssl-dev`, `libdbus-1-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`, and `pkg-config`.

### UX Context

- The product is tray-first: icon opens a compact panel, input is fixed at the top, Enter adds, list appears below, and row trash deletes. [Source: _bmad-output/planning-artifacts/ux-design-specification.md#Core-User-Experience]
- Story 1.1 does not implement the final tray panel UI. It must avoid UI framework choices that would make the Compact Native design harder in later stories.

### Testing Requirements

- Minimum validation for this story is scaffold/build/dev-command validation, not full tray behavior.
- Run or attempt `cargo run`. If system dependencies are missing, record the exact missing package/tool and stop at that actionable blocker.
- Verify generated files are at repo root, not in a nested duplicate project directory.
- Inspect dependency manifests for forbidden Phase 1 technology.
- Leave a concise completion note with the exact Linux validation baseline selected.

### Project Structure Notes

- Root `README.md` should become the source for developer setup and validation baseline.
- Any obsolete Tauri/web scaffold guidance can be removed or rewritten.
- Keep `_bmad-output/` artifacts intact.

### References

- [Source: _bmad-output/planning-artifacts/epics.md#Story-1-1-Scaffold-Rust-Native-App-and-Linux-Validation-Baseline]
- [Source: _bmad-output/planning-artifacts/architecture.md#Selected-Stack-Rust-Native--Slint-UI]
- [Source: _bmad-output/planning-artifacts/architecture.md#Development-Experience]
- [Source: _bmad-output/planning-artifacts/prd.md#Desktop-App-Specific-Requirements]
- [Source: _bmad-output/planning-artifacts/ux-design-specification.md#Platform-Strategy]

## Dev Agent Record

### Agent Model Used

GPT-5

### Debug Log References

- `cargo fmt --check`
- `cargo check`
- `timeout 5s cargo run`

### Completion Notes List

- Created root-level Rust native Cargo scaffold.
- Added Slint UI module with a minimal compact window and add-task callback wiring.
- Added Rust app/task module boundaries for lifecycle, tray preparation, windows, task service, repository, model, and SQLx migrations.
- Added optional `tray` feature for `tray-icon` so base scaffold stays light and does not require GTK/GDK packages until tray validation.
- Documented Debian 13 GNOME X11 validation baseline and Rust native development command in `README.md`.
- Confirmed `cargo fmt --check`, `cargo check`, and controlled `cargo run` pass.

### File List

- `Cargo.toml`
- `Cargo.lock`
- `README.md`
- `migrations/0001_create_tasks.sql`
- `src/main.rs`
- `src/app/errors.rs`
- `src/app/lifecycle.rs`
- `src/app/mod.rs`
- `src/app/state.rs`
- `src/app/tray.rs`
- `src/app/windows.rs`
- `src/ui/mod.rs`
- `src/tasks/migrations.rs`
- `src/tasks/mod.rs`
- `src/tasks/model.rs`
- `src/tasks/repository.rs`
- `src/tasks/service.rs`
- `_bmad-output/implementation-artifacts/1-1-scaffold-rust-native-app-and-linux-validation-baseline.md`
- `_bmad-output/implementation-artifacts/sprint-status.yaml`

### Change Log

- 2026-06-09: Superseded previous Tauri scaffold story with Rust-native Slint scaffold requirements.
- 2026-06-09: Implemented Rust-native Slint scaffold and validated with Cargo.
