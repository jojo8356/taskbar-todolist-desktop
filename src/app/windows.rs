#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum AppWindow {
    TrayPanel,
    FullEdit,
}

pub fn quit_app() {
    let _ = slint::quit_event_loop();
}
