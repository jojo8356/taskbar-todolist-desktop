use tauri::{App, AppHandle, Manager, Runtime, WebviewWindow, Window, WindowEvent};

pub const TRAY_PANEL_LABEL: &str = "main";

pub fn prepare_tray_panel(app: &mut App) -> tauri::Result<()> {
    if let Some(window) = tray_panel(app.handle()) {
        window.hide()?;
    }

    Ok(())
}

pub fn show_tray_panel<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    if let Some(window) = tray_panel(app) {
        window.show()?;
        window.unminimize()?;
        window.set_focus()?;
        window.emit("tray-panel-opened", ())?;
    }

    Ok(())
}

pub fn hide_tray_panel<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    if let Some(window) = tray_panel(app) {
        window.hide()?;
    }

    Ok(())
}

pub fn handle_window_event(window: &Window, event: &WindowEvent) {
    if window.label() != TRAY_PANEL_LABEL {
        return;
    }

    if let WindowEvent::CloseRequested { api, .. } = event {
        api.prevent_close();
        let _ = window.hide();
    }
}

fn tray_panel<R: Runtime>(app: &AppHandle<R>) -> Option<WebviewWindow<R>> {
    app.get_webview_window(TRAY_PANEL_LABEL)
}
