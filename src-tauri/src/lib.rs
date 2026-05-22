mod app;
pub mod tasks;

use tauri::Manager;

#[tauri::command]
fn show_tray_panel(app_handle: tauri::AppHandle) -> Result<(), String> {
    app::windows::show_tray_panel(&app_handle).map_err(|error| error.to_string())
}

#[tauri::command]
fn hide_tray_panel(app_handle: tauri::AppHandle) -> Result<(), String> {
    app::windows::hide_tray_panel(&app_handle).map_err(|error| error.to_string())
}

#[tauri::command]
fn quit_app(app_handle: tauri::AppHandle) {
    app_handle.exit(0);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let state = app::state::initialize(app)?;
            let _ = state.tasks();
            app.manage(state);
            app::windows::prepare_tray_panel(app)?;
            if let Some(tray) = app::tray::create_tray(app)? {
                app.manage(tray);
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            tasks::commands::create_task,
            tasks::commands::list_tasks,
            tasks::commands::update_task,
            tasks::commands::delete_task,
            show_tray_panel,
            hide_tray_panel,
            quit_app
        ])
        .on_window_event(app::windows::handle_window_event)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
