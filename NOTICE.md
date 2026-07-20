# Taskbar Todolist Desktop Notice

This notice explains how to install, use, update, uninstall, and troubleshoot
Taskbar Todolist Desktop.

## Audience

This document is for users and maintainers who want to run the app on a Linux
desktop with a visible tray/status area.

## Quick Start

Install the Debian package:

```bash
sudo apt install ./taskbar-todolist-desktop_0.1.0_amd64.deb
```

Or run the AppImage:

```bash
chmod +x taskbar-todolist-desktop-0.1.0-x86_64.AppImage
./taskbar-todolist-desktop-0.1.0-x86_64.AppImage
```

After launch, look for the app icon in the desktop notification area.

## Basic Use

1. Click the tray icon to open the popup.
2. Type a task in the `Tache` input.
3. Press `Enter` to add the task.
4. Click the checkbox to mark the task as done.
5. Double-click task text to edit it.
6. Press `Enter` while editing to save the change.
7. Click the trash icon to delete a task.
8. Click the settings icon to open language and popup-height options.
9. Click `Hide`, or click the tray icon again, to hide the popup.
10. Click `Quit` or use the tray menu to close the app.

Completed tasks are automatically moved to the bottom of the list.

## Window Behavior

The popup is frameless and always on top. It opens below the tray icon and uses
the tray icon geometry as its anchor, so the popup should not move just because
the click happened slightly left or right inside the same icon.

Clicking the tray icon toggles the popup:

- hidden popup -> opens the popup;
- visible popup -> hides the popup.

The menu item `Open` only opens the popup.

## Data And Privacy

Taskbar Todolist Desktop is local-first.

- No account is required.
- No cloud service is used.
- Tasks are stored in SQLite on the local machine.
- Deleted tasks are soft-deleted with a `deleted_at` timestamp.

Installed/package launches store data under:

```text
${XDG_DATA_HOME:-$HOME/.local/share}/taskbar-todolist-desktop/
```

Development launches from `cargo run` store `taskbar-todolist.sqlite` in the
current project directory.

## Settings

The app creates and maintains this YAML file automatically:

```text
taskbar-todolist.settings.yaml
```

Installed/package launches store it next to the SQLite database:

```text
${XDG_DATA_HOME:-$HOME/.local/share}/taskbar-todolist-desktop/taskbar-todolist.settings.yaml
```

Default content:

```yaml
language: fr
visible_tasks: 3
```

Supported values:

- `language`: `fr` or `en`
- `visible_tasks`: number of task rows shown before scrolling

The maximum `visible_tasks` value is computed when the tray starts:

```text
screen height / 42px task-row pitch
```

For example, a 1080px-high screen allows up to 25 visible task rows. If the
screen height cannot be detected yet, the app temporarily falls back to `20`.

In the settings panel, `visible_tasks` is typed directly and validated with
`Enter`. If the input is not a complete integer, or if it exceeds the current
runtime limit, the field returns to a normalized valid value.

If the file is missing, invalid, or outside supported bounds, the app rewrites it
with normalized values on startup.

## Backup

To back up tasks, copy the SQLite database while the app is stopped:

```bash
cp ~/.local/share/taskbar-todolist-desktop/taskbar-todolist.sqlite ./taskbar-todolist.backup.sqlite
```

To restore a backup:

```bash
mkdir -p ~/.local/share/taskbar-todolist-desktop
cp ./taskbar-todolist.backup.sqlite ~/.local/share/taskbar-todolist-desktop/taskbar-todolist.sqlite
```

## Update

For a `.deb` update:

```bash
sudo apt install ./taskbar-todolist-desktop_0.1.0_amd64.deb
```

For an AppImage update, replace the old AppImage file with the new release file,
then run it again.

The SQLite database is stored separately from the binary and should remain in
place across app updates.

## Uninstall

If installed from the repository with `make install`:

```bash
make uninstall
```

If installed from the Debian package:

```bash
sudo apt remove taskbar-todolist-desktop
```

Removing the app does not necessarily delete the local SQLite database. Remove
the data directory manually only when you are sure you no longer need the tasks:

```bash
rm -rf ~/.local/share/taskbar-todolist-desktop
```

## Runtime Requirements

Required desktop/runtime libraries:

- GTK 3
- Ayatana AppIndicator or compatible tray support
- SQLite
- `notify-send` for startup notifications

Debian/Ubuntu runtime packages:

```bash
sudo apt install libgtk-3-0 libayatana-appindicator3-1 libsqlite3-0 libnotify-bin
```

Build packages for source builds:

```bash
sudo apt install build-essential pkg-config libgtk-3-dev libayatana-appindicator3-dev libsqlite3-dev libnotify-bin
```

## Developer Commands

Development run:

```bash
./run_dev.sh
```

Release run:

```bash
./run_prod.sh
```

Watch mode:

```bash
cargo install cargo-watch
./run_watch.sh
```

Test suite:

```bash
cargo test
```

Package build:

```bash
make clean-dist package
```

Rust web documentation:

```bash
make docs
```

The generated entry point is:

```text
target/doc/taskbar_todolist_desktop/index.html
```

## Logs

Enable trace logging:

```bash
RUST_LOG=taskbar_todolist_desktop=trace ./run_dev.sh
```

Useful log areas:

- `taskbar_todolist_desktop::app::tray` for tray icon, popup positioning, show/hide behavior;
- `taskbar_todolist_desktop::ui` for UI callbacks and task model updates;
- `taskbar_todolist_desktop::tasks::repository` for SQLite operations;
- `taskbar_todolist_desktop::tasks::service` for task service calls.

## Known Limits

- Linux-only release artifacts are provided.
- The app expects a working tray/status area in the desktop environment.
- The AppImage may still require common GTK/AppIndicator libraries from the host.
- There is no mobile sync or cloud sync in this release.
- There is no multi-list/project-management feature set in this release.

## Release Artifacts

Release v0.1.0 artifacts:

- `taskbar-todolist-desktop_0.1.0_amd64.deb`
- `taskbar-todolist-desktop-0.1.0-x86_64.AppImage`

Published release:

```text
https://github.com/taskbar-todolist/taskbar-todolist-desktop/releases/tag/v0.1.0
```
