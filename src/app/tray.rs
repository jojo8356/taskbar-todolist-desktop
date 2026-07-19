use std::error::Error;
use std::ffi::CString;
use std::fmt;
use std::mem::transmute;
use std::os::raw::{c_int, c_uint};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use gtk::glib::signal::{SignalHandlerId, connect_raw};
use gtk::glib::translate::{IntoGlib, from_glib};
use gtk::prelude::*;
use slint::{ComponentHandle, PhysicalPosition, Weak};

use crate::app::windows;
use crate::ui::MainWindow;

const TRAY_ICON_NAME: &str = "task-due";
const TRAY_TOOLTIP: &str = "Taskbar Todolist";
const PANEL_WIDTH: i32 = 320;
const PANEL_GAP: i32 = 6;

pub struct AppTray {
    _thread: thread::JoinHandle<()>,
}

pub fn create_tray(window: Weak<MainWindow>) -> Option<AppTray> {
    tracing::trace!("create_tray");
    match create_app_tray(window) {
        Ok(tray) => Some(tray),
        Err(error) => {
            tracing::error!(error = %error, "failed to create tray");
            eprintln!("{error}");
            None
        }
    }
}

fn create_app_tray(window: Weak<MainWindow>) -> Result<AppTray, TrayError> {
    tracing::trace!("create_app_tray");
    let (ready_sender, ready_receiver) = mpsc::channel();
    let thread = thread::spawn(move || {
        tracing::trace!("gtk tray thread started");
        let result = run_gtk_tray(window);
        let is_ok = result.is_ok();
        let _ = ready_sender.send(result);

        if is_ok {
            tracing::debug!("gtk tray initialized, entering gtk loop");
            run_gtk_loop();
        }
    });

    ready_receiver.recv().map_err(TrayError::ThreadStartup)??;

    Ok(AppTray { _thread: thread })
}

fn run_gtk_tray(window: Weak<MainWindow>) -> Result<(), TrayError> {
    tracing::trace!("run_gtk_tray");
    let gtk = GtkInitialized::initialize()?;
    let menu = build_tray_menu(&gtk, window.clone());
    let tray = LegacyStatusIcon::new(TRAY_ICON_NAME, TRAY_TOOLTIP, menu, window)?;

    Box::leak(Box::new(tray));

    Ok(())
}

struct GtkInitialized;

impl GtkInitialized {
    fn initialize() -> Result<Self, TrayError> {
        tracing::trace!("GtkInitialized::initialize");
        gtk::init().map_err(TrayError::Gtk)?;
        tracing::debug!("gtk initialized");
        Ok(Self)
    }
}

struct LegacyStatusIcon {
    icon: *mut gtk::ffi::GtkStatusIcon,
    _menu: gtk::Menu,
    _handlers: Vec<SignalHandlerId>,
}

impl LegacyStatusIcon {
    fn new(
        icon_name: &str,
        tooltip: &str,
        menu: gtk::Menu,
        window: Weak<MainWindow>,
    ) -> Result<Self, TrayError> {
        tracing::trace!(icon_name, tooltip, "LegacyStatusIcon::new");
        let icon_name = CString::new(icon_name).map_err(TrayError::InvalidCString)?;
        let tooltip = CString::new(tooltip).map_err(TrayError::InvalidCString)?;

        let icon = unsafe { gtk::ffi::gtk_status_icon_new_from_icon_name(icon_name.as_ptr()) };
        if icon.is_null() {
            return Err(TrayError::StatusIconUnavailable);
        }

        unsafe {
            gtk::ffi::gtk_status_icon_set_title(icon, tooltip.as_ptr());
            gtk::ffi::gtk_status_icon_set_tooltip_text(icon, tooltip.as_ptr());
            gtk::ffi::gtk_status_icon_set_visible(icon, true.into_glib());
        }
        tracing::debug!("gtk status icon visible");

        let mut handlers = Vec::with_capacity(3);
        let click_window = window.clone();
        handlers.push(unsafe {
            connect_status_icon_button_press(icon, move |icon, event| {
                tracing::trace!(
                    button = event.button,
                    x = event.x_root,
                    y = event.y_root,
                    "status icon button press"
                );
                if event.button == 1 {
                    let anchor = popup_anchor_from_icon_or_event(status_icon_area(icon), event);
                    toggle_panel_at(click_window.clone(), Some(anchor));
                    true
                } else {
                    false
                }
            })
        });

        handlers.push(unsafe {
            connect_status_icon_activate(icon, move |icon| {
                tracing::trace!("status icon activate");
                toggle_panel_at(window.clone(), status_icon_anchor(icon))
            })
        });

        let popup_menu = menu.clone();
        handlers.push(unsafe {
            connect_status_icon_popup_menu(icon, move |button, activate_time| {
                tracing::trace!(button, activate_time, "status icon popup menu");
                popup_menu.popup_easy(button, activate_time);
            })
        });

        Ok(Self {
            icon,
            _menu: menu,
            _handlers: handlers,
        })
    }
}

impl Drop for LegacyStatusIcon {
    fn drop(&mut self) {
        unsafe {
            gtk::ffi::gtk_status_icon_set_visible(self.icon, false.into_glib());
            gtk::glib::gobject_ffi::g_object_unref(self.icon.cast());
        }
    }
}

fn build_tray_menu(_gtk: &GtkInitialized, window: Weak<MainWindow>) -> gtk::Menu {
    tracing::trace!("build_tray_menu");
    let menu = gtk::Menu::new();
    let open_item = gtk::MenuItem::with_label("Open");
    let quit_item = gtk::MenuItem::with_label("Quit");

    open_item.connect_activate(move |_| show_panel_at(window.clone(), None));
    quit_item.connect_activate(|_| {
        let _ = slint::invoke_from_event_loop(windows::quit_app);
    });

    menu.append(&open_item);
    menu.append(&quit_item);
    menu.show_all();

    menu
}

unsafe fn connect_status_icon_activate<F>(
    icon: *mut gtk::ffi::GtkStatusIcon,
    callback: F,
) -> SignalHandlerId
where
    F: Fn(*mut gtk::ffi::GtkStatusIcon) + 'static,
{
    unsafe extern "C" fn trampoline<F>(
        icon: *mut gtk::ffi::GtkStatusIcon,
        callback: gtk::glib::ffi::gpointer,
    ) where
        F: Fn(*mut gtk::ffi::GtkStatusIcon) + 'static,
    {
        let callback = unsafe { &*(callback as *const F) };
        callback(icon);
    }

    let callback = Box::new(callback);
    unsafe {
        connect_raw(
            icon.cast(),
            b"activate\0".as_ptr().cast(),
            Some(transmute::<_, unsafe extern "C" fn()>(
                trampoline::<F> as *const (),
            )),
            Box::into_raw(callback),
        )
    }
}

unsafe fn connect_status_icon_button_press<F>(
    icon: *mut gtk::ffi::GtkStatusIcon,
    callback: F,
) -> SignalHandlerId
where
    F: Fn(*mut gtk::ffi::GtkStatusIcon, TrayButtonEvent) -> bool + 'static,
{
    unsafe extern "C" fn trampoline<F>(
        icon: *mut gtk::ffi::GtkStatusIcon,
        event: *mut gtk::gdk::ffi::GdkEventButton,
        callback: gtk::glib::ffi::gpointer,
    ) -> gtk::glib::ffi::gboolean
    where
        F: Fn(*mut gtk::ffi::GtkStatusIcon, TrayButtonEvent) -> bool + 'static,
    {
        if event.is_null() {
            return false.into_glib();
        }

        let callback = unsafe { &*(callback as *const F) };
        let event = unsafe {
            TrayButtonEvent {
                button: (*event).button,
                x_root: (*event).x_root.round() as c_int,
                y_root: (*event).y_root.round() as c_int,
            }
        };

        callback(icon, event).into_glib()
    }

    let callback = Box::new(callback);
    unsafe {
        connect_raw(
            icon.cast(),
            b"button-press-event\0".as_ptr().cast(),
            Some(transmute::<_, unsafe extern "C" fn()>(
                trampoline::<F> as *const (),
            )),
            Box::into_raw(callback),
        )
    }
}

unsafe fn connect_status_icon_popup_menu<F>(
    icon: *mut gtk::ffi::GtkStatusIcon,
    callback: F,
) -> SignalHandlerId
where
    F: Fn(u32, u32) + 'static,
{
    unsafe extern "C" fn trampoline<F>(
        _icon: *mut gtk::ffi::GtkStatusIcon,
        button: c_uint,
        activate_time: u32,
        callback: gtk::glib::ffi::gpointer,
    ) where
        F: Fn(u32, u32) + 'static,
    {
        let callback = unsafe { &*(callback as *const F) };
        callback(button, activate_time);
    }

    let callback = Box::new(callback);
    unsafe {
        connect_raw(
            icon.cast(),
            b"popup-menu\0".as_ptr().cast(),
            Some(transmute::<_, unsafe extern "C" fn()>(
                trampoline::<F> as *const (),
            )),
            Box::into_raw(callback),
        )
    }
}

fn run_gtk_loop() {
    loop {
        while gtk::events_pending() {
            gtk::main_iteration_do(false);
        }

        thread::sleep(Duration::from_millis(50));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ScreenPoint {
    x: i32,
    y: i32,
}

impl ScreenPoint {
    fn from_event(event: TrayButtonEvent) -> Self {
        Self {
            x: event.x_root,
            y: event.y_root,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TrayButtonEvent {
    button: c_uint,
    x_root: i32,
    y_root: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TrayIconArea {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

fn status_icon_anchor(icon: *mut gtk::ffi::GtkStatusIcon) -> Option<ScreenPoint> {
    tracing::trace!("status_icon_anchor");
    status_icon_area(icon).map(popup_anchor_from_icon)
}

fn status_icon_area(icon: *mut gtk::ffi::GtkStatusIcon) -> Option<TrayIconArea> {
    tracing::trace!("status_icon_area");
    let mut screen = std::ptr::null_mut();
    let mut area = gtk::gdk::ffi::GdkRectangle {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut orientation = gtk::ffi::GTK_ORIENTATION_HORIZONTAL;

    let has_geometry = unsafe {
        from_glib(gtk::ffi::gtk_status_icon_get_geometry(
            icon,
            &mut screen,
            &mut area,
            &mut orientation,
        ))
    };

    tracing::debug!(
        has_geometry,
        x = area.x,
        y = area.y,
        width = area.width,
        height = area.height,
        "status icon geometry"
    );

    (has_geometry && area.width > 0 && area.height > 0).then(|| TrayIconArea {
        x: area.x,
        y: area.y,
        width: area.width,
        height: area.height,
    })
}

fn popup_anchor_from_icon(icon_area: TrayIconArea) -> ScreenPoint {
    ScreenPoint {
        x: icon_area.x + icon_area.width / 2,
        y: icon_area.y + icon_area.height,
    }
}

fn popup_anchor_from_icon_or_event(
    icon_area: Option<TrayIconArea>,
    event: TrayButtonEvent,
) -> ScreenPoint {
    icon_area
        .map(popup_anchor_from_icon)
        .unwrap_or_else(|| ScreenPoint::from_event(event))
}

fn popup_position_from_anchor(anchor: ScreenPoint) -> PhysicalPosition {
    PhysicalPosition::new(
        (anchor.x - PANEL_WIDTH / 2).max(0),
        (anchor.y + PANEL_GAP).max(0),
    )
}

fn show_panel_at(window: Weak<MainWindow>, anchor: Option<ScreenPoint>) {
    tracing::trace!(?anchor, "show_panel_at");
    let _ = slint::invoke_from_event_loop(move || {
        if let Some(window) = window.upgrade() {
            show_panel(&window, anchor);
        }
    });
}

fn toggle_panel_at(window: Weak<MainWindow>, anchor: Option<ScreenPoint>) {
    tracing::trace!(?anchor, "toggle_panel_at");
    let _ = slint::invoke_from_event_loop(move || {
        if let Some(window) = window.upgrade() {
            if window.window().is_visible() {
                tracing::debug!(?anchor, "hiding visible slint panel from tray click");
                let _ = window.hide();
            } else {
                show_panel(&window, anchor);
            }
        }
    });
}

fn show_panel(window: &MainWindow, anchor: Option<ScreenPoint>) {
    let position = anchor.map(popup_position_from_anchor);
    tracing::debug!(?anchor, ?position, "showing slint panel");

    let _ = window.show();
    if let Some(position) = position {
        window.window().set_position(position);
    }
    window.window().request_redraw();
}

#[derive(Debug)]
pub enum TrayError {
    Gtk(gtk::glib::BoolError),
    InvalidCString(std::ffi::NulError),
    StatusIconUnavailable,
    ThreadStartup(mpsc::RecvError),
}

impl fmt::Display for TrayError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Gtk(error) => write!(formatter, "GTK initialization error: {error}"),
            Self::InvalidCString(error) => {
                write!(formatter, "tray text contains a null byte: {error}")
            }
            Self::StatusIconUnavailable => write!(formatter, "GTK status icon creation failed"),
            Self::ThreadStartup(error) => write!(formatter, "tray thread startup error: {error}"),
        }
    }
}

impl Error for TrayError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tray_menu_construction_requires_gtk_initialization_guard() {
        fn requires_guard(_: fn(&GtkInitialized, Weak<MainWindow>) -> gtk::Menu) {}

        requires_guard(build_tray_menu);
    }

    #[test]
    fn tray_icon_uses_theme_icon_name() {
        assert_eq!(TRAY_ICON_NAME, "task-due");
    }

    #[test]
    fn popup_position_opens_below_tray_icon() {
        assert_eq!(
            popup_position_from_anchor(popup_anchor_from_icon(TrayIconArea {
                x: 170,
                y: 0,
                width: 24,
                height: 24,
            })),
            PhysicalPosition::new(22, 30)
        );
    }

    #[test]
    fn popup_position_is_stable_across_clicks_inside_same_icon() {
        let icon_area = TrayIconArea {
            x: 1730,
            y: 0,
            width: 24,
            height: 24,
        };
        let left_click = TrayButtonEvent {
            button: 1,
            x_root: 1732,
            y_root: 17,
        };
        let right_click = TrayButtonEvent {
            button: 1,
            x_root: 1752,
            y_root: 17,
        };

        assert_ne!(
            popup_position_from_anchor(ScreenPoint::from_event(left_click)),
            popup_position_from_anchor(ScreenPoint::from_event(right_click))
        );
        assert_eq!(
            popup_position_from_anchor(popup_anchor_from_icon_or_event(
                Some(icon_area),
                left_click
            )),
            popup_position_from_anchor(popup_anchor_from_icon_or_event(
                Some(icon_area),
                right_click
            ))
        );
    }

    #[test]
    fn popup_position_clamps_left_edge_to_screen() {
        assert_eq!(
            popup_position_from_anchor(popup_anchor_from_icon(TrayIconArea {
                x: 4,
                y: 0,
                width: 24,
                height: 24,
            })),
            PhysicalPosition::new(0, 30)
        );
    }

    #[test]
    fn left_click_event_produces_screen_anchor() {
        assert_eq!(
            ScreenPoint::from_event(TrayButtonEvent {
                button: 1,
                x_root: 1743,
                y_root: 15,
            }),
            ScreenPoint { x: 1743, y: 15 }
        );
    }
}
