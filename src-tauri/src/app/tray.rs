use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    App, Error,
};

use crate::app::windows;

pub fn create_tray(app: &mut App) -> tauri::Result<()> {
    let open = MenuItem::with_id(app, "open", "Open Taskbar Todolist", true, None::<&str>)?;
    let hide = MenuItem::with_id(app, "hide", "Hide Panel", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let menu = Menu::with_items(app, &[&open, &hide, &separator, &quit])?;

    let icon = app
        .default_window_icon()
        .ok_or_else(|| Error::AssetNotFound("default tray icon".into()))?;

    TrayIconBuilder::new()
        .icon(icon.clone())
        .tooltip("Taskbar Todolist")
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "open" => {
                let _ = windows::show_tray_panel(app);
            }
            "hide" => {
                let _ = windows::hide_tray_panel(app);
            }
            "quit" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let _ = windows::show_tray_panel(tray.app_handle());
            }
        })
        .build(app)?;

    Ok(())
}
