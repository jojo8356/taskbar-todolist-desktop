# Story 2.1: Compact Native Tray Panel UI

Status: done

## Story

As a desktop user,
I want a compact native-feeling tray panel,
so that task capture feels like a lightweight system utility.

## Acceptance Criteria

1. Given the tray panel opens, when it is rendered, then it follows the Compact Native visual direction and is approximately 320px to 380px wide with compact spacing.
2. Given the panel renders, when no tasks are active, then the input remains visible at the top and a minimal empty state is shown without onboarding-heavy text.
3. Given the panel UI is implemented, when visual styles are inspected, then the light theme uses the documented background, panel, border, text, accent, danger, and danger hover tokens and no sidebar, dashboard, nested cards, decorative cards, or decorative gradients are introduced.
4. Given the panel is keyboard accessible, when the input receives focus, then the focus state is visible.

## Tasks / Subtasks

- [x] Verify compact tray shell dimensions and spacing. (AC: 1)
- [x] Keep input at the top and preserve minimal empty state. (AC: 2)
- [x] Align light theme colors to the documented Compact Native tokens. (AC: 3)
- [x] Preserve visible input focus treatment. (AC: 4)
- [x] Run validation. (AC: 1-4)
  - [x] `cargo fmt --manifest-path src-tauri/Cargo.toml --check`
  - [x] `cargo test --manifest-path src-tauri/Cargo.toml`
  - [x] `pnpm build`

## Dev Notes

- Epic 3 already introduced the persisted tray task store, top input, active task region, and row rendering. This story tightens the visual layer to match the UX spec.
- Compact Native tokens come from `_bmad-output/planning-artifacts/ux-design-specification.md#Visual-Design-Foundation`.
- Tray panel dimensions remain driven by Tauri window size `340x420` and CSS constraints `320px` to `380px`.

## Dev Agent Record

### Agent Model Used

GPT-5

### Debug Log References

- `cargo fmt --manifest-path src-tauri/Cargo.toml --check`
- `env PKG_CONFIG_PATH=/tmp/taskbar-todolist-native-pkgconfig PKG_CONFIG_SYSROOT_DIR=/tmp/taskbar-todolist-native-root LD_LIBRARY_PATH=/tmp/taskbar-todolist-native-root/usr/lib/x86_64-linux-gnu cargo test --manifest-path src-tauri/Cargo.toml`
- `pnpm build`

### Completion Notes List

- Confirmed tray shell already meets compact size and layout constraints.
- Replaced earlier teal/slate palette with the documented Compact Native light theme tokens.
- Preserved quiet utility styling, no sidebar, no nested cards, and no decorative gradients.
- Preserved visible input focus with the documented blue accent token.

### File List

- `_bmad-output/implementation-artifacts/2-1-compact-native-tray-panel-ui.md`
- `_bmad-output/implementation-artifacts/sprint-status.yaml`
- `src/styles.css`
