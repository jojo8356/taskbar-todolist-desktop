pub fn prepare_tray_validation() {
    // Story 1.2 will create the actual tray icon/menu on the Linux validation baseline.
    #[cfg(feature = "tray")]
    let _crate_is_available = std::any::type_name::<tray_icon::TrayIcon>();
}
