# Story 1.1: Scaffold Tauri App and Linux Validation Baseline

Status: ready-for-dev

<!-- Note: Validation is optional. Run validate-create-story for quality check before dev-story. -->

## Story

As a desktop user,
I want the app scaffolded with the selected Tauri starter and a named Linux validation baseline,
so that implementation starts from the agreed architecture and proves the target environment first.

## Acceptance Criteria

1. Given the repository is ready for implementation, when the app is scaffolded, then it uses the official Tauri Vanilla TypeScript starter and the project can run through the Tauri development command.
2. Given the starter has been scaffolded, when dependencies are installed, then dependency installation completes successfully and the repository documents the local development command needed to run the Tauri app.
3. Given the architecture requires a Linux tray validation baseline, when the scaffold story is completed, then the chosen MVP Linux desktop environment and tray/AppIndicator setup are documented in the repository and the validation baseline is referenced by later tray lifecycle tests.
4. Given Phase 1 must remain desktop-local, when dependencies are added, then no backend, cloud service, account provider, mobile framework, sync runtime, Prisma, Drizzle, Kysely, MikroORM, TypeORM, or Node ORM sidecar is introduced.

## Tasks / Subtasks

- [ ] Scaffold the app with the official Tauri Vanilla TypeScript starter. (AC: 1)
  - [ ] Use the project root as the final app root; avoid creating a nested app directory that leaves the repo root as a wrapper.
  - [ ] Select `TypeScript / JavaScript`, `pnpm`, `Vanilla`, and `TypeScript`.
  - [ ] Preserve existing BMad artifacts, docs, and repo history while replacing only obsolete placeholder scaffold files as needed.
- [ ] Install and verify baseline dependencies. (AC: 2, 4)
  - [ ] Run dependency installation with `pnpm`.
  - [ ] Verify the Tauri dev command starts or reaches the next actionable missing-system-dependency error.
  - [ ] Do not add React, Vue, Svelte, Tailwind, SQLite, SQLx, sync, backend, mobile, cloud, account, or Node ORM dependencies in this story.
- [ ] Document the local development command and Linux validation baseline. (AC: 2, 3)
  - [ ] Update the root README so it reflects the active BMad decisions: Linux-first, Tauri v2, Vanilla TypeScript, Phase 1 desktop-local, no mandatory backend/cloud/sync.
  - [ ] Add a concise validation baseline section naming the exact desktop environment/session used for MVP tray/AppIndicator validation, for example KDE Plasma X11/Wayland or GNOME with AppIndicator extension.
  - [ ] Document required Linux system packages at least for Debian/Ubuntu-family environments if that matches the chosen baseline.
- [ ] Remove or rewrite obsolete repo guidance. (AC: 3, 4)
  - [ ] Replace the old `src/README.md` guidance that points to `src/tray`, `src/ui`, `src/storage`, and `src/sync`.
  - [ ] Ensure docs do not claim Phase 1 uses a backend, sync runtime, or mobile framework.
- [ ] Perform story validation. (AC: 1-4)
  - [ ] Confirm the scaffolded app has `src-tauri/`, frontend `src/`, `package.json`, and Tauri/Vite config files in the expected root.
  - [ ] Confirm `pnpm tauri dev` is the documented local dev command.
  - [ ] Confirm `package.json` and `src-tauri/Cargo.toml` do not include forbidden Phase 1 dependencies listed in AC4.

## Dev Notes

### Current Repository State

- The repository is still pre-scaffold. Current source files are only root `README.md`, `src/README.md`, BMad planning artifacts, and basic repo metadata.
- Root `README.md` and `src/README.md` contain stale pre-BMad concepts: backend sync, `src/tray`, `src/storage`, and `src/sync`. Treat these as obsolete and update them during this story.
- No previous implementation story exists. There are no established code patterns to preserve beyond BMad planning artifacts and repo history.

### Architecture Guardrails

- Use Tauri v2 as the desktop shell and the official Tauri Vanilla TypeScript starter as the implementation foundation. [Source: _bmad-output/planning-artifacts/architecture.md#Selected-Starter-Official-Tauri-Vanilla-TypeScript]
- Use Vanilla TypeScript frontend modules in Phase 1. Do not introduce React, Vue, or Svelte unless a later architecture change explicitly permits it. [Source: _bmad-output/planning-artifacts/architecture.md#Frontend-Architecture]
- Keep Phase 1 offline-only, backend-free, cloud-free, account-free, mobile-free, and sync-free. [Source: _bmad-output/planning-artifacts/architecture.md#Core-Architectural-Decisions]
- Do not add Prisma, Drizzle, Kysely, MikroORM, TypeORM, or a Node ORM sidecar. Persistence will be Rust-side SQLite with SQLx in later Epic 3 stories, not in this scaffold story. [Source: _bmad-output/planning-artifacts/architecture.md#Persistence-Technology-Decision-Notes]
- Do not add Tailwind CSS in this story. Architecture says Tailwind v4 comes after base Tauri startup is proven. [Source: _bmad-output/planning-artifacts/architecture.md#Development-Experience]
- First technical risk is Linux tray/AppIndicator feasibility; this story must name the validation baseline so Story 1.2 can prove tray lifecycle against it. [Source: _bmad-output/planning-artifacts/architecture.md#Gap-Analysis-Results]

### Expected Project Shape After Scaffold

The official starter may generate a minimal layout first. Long-term architecture expects this shape to evolve:

```text
src/
  main.ts
  ui/
  state/
  types/
  styles/
src-tauri/
  Cargo.toml
  tauri.conf.json
  capabilities/
  icons/
  src/
    main.rs
```

Do not create the later task-specific folders yet unless the starter requires adjacent files. Stories 1.2, 1.3, and Epic 3 will add tray lifecycle and persistence modules.

### Linux Validation Baseline Requirements

The baseline must be concrete enough for later test reproduction:

- Linux distribution and version.
- Desktop environment and session type if known, e.g. GNOME Wayland, GNOME X11, KDE Plasma Wayland, KDE Plasma X11.
- Tray/AppIndicator support mechanism, e.g. built-in tray support or GNOME AppIndicator extension.
- Required system packages installed or expected.
- Manual validation command, expected outcome, and known limitations.

Do not write "Linux" as the baseline by itself; that is too vague for NFR17.

### Latest Technical Information

- Official Tauri docs recommend `pnpm create tauri-app` and support the Vanilla TypeScript template through prompts: frontend language `TypeScript / JavaScript`, package manager `pnpm`, UI template `Vanilla`, UI flavor `TypeScript`. Source: https://v2.tauri.app/start/create-project/
- Tauri Linux prerequisites include WebKitGTK 4.1 development packages, build tooling, `libxdo-dev`, `libssl-dev`, `libayatana-appindicator3-dev`, and `librsvg2-dev` on Debian/Ubuntu-family systems. Source: https://v2.tauri.app/start/prerequisites/
- Tauri tray support requires the `tray-icon` feature when implementing tray behavior. That belongs in Story 1.2 unless the starter already configures it. Source: https://v2.tauri.app/learn/system-tray/
- Tauri docs note Linux tray click events are unsupported even when the icon is shown; it may still show a context menu on right click. Story 1.2 must account for this with menu-based or platform-appropriate validation rather than assuming JS/Rust click event parity on Linux. Source: https://v2.tauri.app/learn/system-tray/
- Vite current docs list `vanilla-ts` as a supported template and note current Vite requires Node.js `20.19+` or `22.12+`. If local Node is older, document the blocker instead of changing the architecture. Source: https://vite.dev/guide/

### UX Context

- The product is tray-first: icon opens a compact panel, input is fixed at the top, Enter adds, list appears below, and row trash deletes. [Source: _bmad-output/planning-artifacts/ux-design-specification.md#Core-User-Experience]
- Story 1.1 does not implement the final tray panel UI. It must avoid UI framework choices that would make the Compact Native design harder in later stories.

### Testing Requirements

- Minimum validation for this story is scaffold/build/dev-command validation, not full tray behavior.
- Run or attempt `pnpm tauri dev`. If system dependencies are missing, record the exact missing package/tool and stop at that actionable blocker.
- Verify generated files are at repo root, not in a nested duplicate project directory.
- Inspect dependency manifests for forbidden Phase 1 technology.
- Leave a concise completion note with the exact Linux validation baseline selected.

### Project Structure Notes

- Root `README.md` should become the source for developer setup and validation baseline.
- `src/README.md` can be removed by the scaffold or rewritten; do not preserve its obsolete `sync` guidance.
- Keep `_bmad-output/` artifacts intact.

### References

- [Source: _bmad-output/planning-artifacts/epics.md#Story-1-1-Scaffold-Tauri-App-and-Linux-Validation-Baseline]
- [Source: _bmad-output/planning-artifacts/architecture.md#Selected-Starter-Official-Tauri-Vanilla-TypeScript]
- [Source: _bmad-output/planning-artifacts/architecture.md#Development-Experience]
- [Source: _bmad-output/planning-artifacts/prd.md#Desktop-App-Specific-Requirements]
- [Source: _bmad-output/planning-artifacts/ux-design-specification.md#Platform-Strategy]
- [Official Tauri Create Project](https://v2.tauri.app/start/create-project/)
- [Official Tauri Linux Prerequisites](https://v2.tauri.app/start/prerequisites/)
- [Official Tauri System Tray](https://v2.tauri.app/learn/system-tray/)
- [Official Vite Guide](https://vite.dev/guide/)

## Dev Agent Record

### Agent Model Used

TBD by dev agent.

### Debug Log References

### Completion Notes List

- Ultimate context engine analysis completed - comprehensive developer guide created.

### File List

- `_bmad-output/implementation-artifacts/1-1-scaffold-tauri-app-and-linux-validation-baseline.md`
