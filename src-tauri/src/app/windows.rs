use tauri::{
    App, AppHandle, Emitter, Manager, Runtime, WebviewUrl, WebviewWindow, WebviewWindowBuilder,
    Window, WindowEvent,
};

pub const TRAY_PANEL_LABEL: &str = "main";

pub fn prepare_tray_panel(app: &mut App) -> tauri::Result<()> {
    if let Some(window) = tray_panel(app.handle()) {
        window.hide()?;
    }

    Ok(())
}

pub fn show_tray_panel<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    let window = match tray_panel(app) {
        Some(window) => window,
        None => create_tray_panel(app)?,
    };

    window.show()?;
    window.unminimize()?;
    window.set_focus()?;
    window.emit("tray-panel-opened", ())?;

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

fn create_tray_panel<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<WebviewWindow<R>> {
    WebviewWindowBuilder::new(app, TRAY_PANEL_LABEL, WebviewUrl::App("index.html".into()))
        .title("Taskbar Todolist")
        .inner_size(340.0, 420.0)
        .resizable(false)
        .visible(false)
        .build()
}
