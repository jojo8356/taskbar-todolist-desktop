mod app;
mod tasks;
mod ui;

use slint::ComponentHandle;

fn main() -> Result<(), slint::PlatformError> {
    let app_state = app::AppState::new();
    let main_window = ui::create_main_window(&app_state)?;
    let _tray = app::tray::create_tray(main_window.as_weak());

    main_window.run()
}
