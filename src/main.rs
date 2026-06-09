mod app;
mod tasks;
mod ui;

use slint::ComponentHandle;

fn main() -> Result<(), slint::PlatformError> {
    let app_state = app::AppState::new();
    let main_window = ui::create_main_window(&app_state)?;

    app::tray::prepare_tray_validation();
    main_window.run()
}
