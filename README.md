# Taskbar Todolist Desktop

Linux-first desktop app for a tray-first local todolist. Phase 1 is desktop-local only: no backend, no cloud service, no account provider, no mobile framework, and no sync runtime.

## Stack

- Tauri v2 desktop shell.
- Vanilla TypeScript frontend generated from the official Tauri starter.
- pnpm for JavaScript dependencies.
- Rust for the Tauri application shell.

Do not add React, Vue, Svelte, Tailwind, backend/cloud/sync dependencies, mobile runtime dependencies, Prisma, Drizzle, Kysely, MikroORM, TypeORM, or a Node ORM sidecar during Phase 1 scaffold work.

## Local Development

Install JavaScript dependencies:

```sh
pnpm install
```

Run the Tauri app in development:

```sh
pnpm tauri dev
```

On Linux, tray icon click behavior depends on the desktop environment. The
validated fallback path for the tray lifecycle is the tray menu item
`Open Taskbar Todolist`, which opens the compact panel through Rust/Tauri.
The panel UI itself is Vanilla TypeScript and uses Lucide icons only for
in-panel visual controls.

The frontend-only Vite command is available for quick UI checks:

```sh
pnpm dev
```

## Linux Validation Baseline

MVP tray/AppIndicator validation target:

- Distribution: Debian GNU/Linux 13 (trixie).
- Desktop environment: GNOME.
- Session type: X11.
- Tray/AppIndicator support: GNOME AppIndicator/KStatusNotifierItem extension must be enabled for tray validation.
- Manual validation command: `pnpm tauri dev`.
- Expected scaffold outcome: Vite starts on `http://localhost:1420/`, then Cargo starts the Tauri desktop process.

Required Debian/Ubuntu-family system packages for Tauri v2 desktop development:

```sh
sudo apt install \
  build-essential \
  curl \
  file \
  libdbus-1-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  libssl-dev \
  libwebkit2gtk-4.1-dev \
  libxdo-dev \
  pkg-config
```

Current local validation note: rustup is installed for the user and the project
now validates with `rustc 1.95.0` / `cargo 1.95.0`. `pnpm tauri dev`
starts Vite and reaches native compilation. The current actionable system
blocker is missing `dbus-1.pc`; install `libdbus-1-dev` and `pkg-config` on
Debian/Ubuntu-family systems before continuing native tray validation.
