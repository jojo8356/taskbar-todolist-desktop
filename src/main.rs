//! Entrypoint for the Linux tray todo app.
//!
//! Startup intentionally creates the Slint window before the GTK tray: the tray
//! owns the user's open/hide action, while the window starts hidden in the
//! background and is positioned only after a tray click.

mod app;
mod tasks;
mod ui;

use slint::ComponentHandle;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    app::tracing::init();
    tracing::info!("starting taskbar todolist desktop");

    let app_state = app::AppState::new()?;
    let active_task_count = app_state
        .tasks
        .list_active_tasks()
        .unwrap_or_default()
        .len();
    tracing::debug!(
        active_task_count,
        "active tasks loaded before window creation"
    );

    let main_window = ui::create_main_window(&app_state)?;
    let _tray = app::tray::create_tray(main_window.as_weak());

    app::notifications::show_todolist_initialized(active_task_count);
    tracing::info!("entering slint event loop");
    slint::run_event_loop_until_quit()?;

    tracing::info!("slint event loop exited");
    Ok(())
}
