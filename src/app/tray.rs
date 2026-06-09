use std::{error::Error, fmt};

use slint::{ComponentHandle, Weak};
use zbus::{Connection, interface};

use crate::ui::MainWindow;

const ITEM_PATH: &str = "/StatusNotifierItem";
const WATCHER_BUS_NAME: &str = "org.kde.StatusNotifierWatcher";
const WATCHER_PATH: &str = "/StatusNotifierWatcher";
const WATCHER_INTERFACE: &str = "org.kde.StatusNotifierWatcher";

pub struct StatusNotifierTray {
    _runtime: tokio::runtime::Runtime,
    _connection: Connection,
}

pub fn create_tray(window: Weak<MainWindow>) -> Option<StatusNotifierTray> {
    match create_status_notifier_tray(window) {
        Ok(tray) => Some(tray),
        Err(error) => {
            eprintln!("{error}");
            None
        }
    }
}

fn create_status_notifier_tray(window: Weak<MainWindow>) -> Result<StatusNotifierTray, TrayError> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(TrayError::Runtime)?;

    let connection = runtime.block_on(async move {
        let connection = Connection::session().await?;
        connection
            .object_server()
            .at(ITEM_PATH, StatusNotifierItem { window })
            .await?;

        let Some(unique_name) = connection.unique_name() else {
            return Err(TrayError::MissingUniqueBusName);
        };

        connection
            .call_method(
                Some(WATCHER_BUS_NAME),
                WATCHER_PATH,
                Some(WATCHER_INTERFACE),
                "RegisterStatusNotifierItem",
                &(unique_name.as_str()),
            )
            .await?;

        Ok::<Connection, TrayError>(connection)
    })?;

    Ok(StatusNotifierTray {
        _runtime: runtime,
        _connection: connection,
    })
}

struct StatusNotifierItem {
    window: Weak<MainWindow>,
}

impl StatusNotifierItem {
    fn show_panel(&self) {
        let window = self.window.clone();
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(window) = window.upgrade() {
                let _ = window.show();
                window.window().request_redraw();
            }
        });
    }
}

#[interface(name = "org.kde.StatusNotifierItem")]
impl StatusNotifierItem {
    async fn activate(&self, _x: i32, _y: i32) {
        self.show_panel();
    }

    async fn context_menu(&self, _x: i32, _y: i32) {
        self.show_panel();
    }

    async fn secondary_activate(&self, _x: i32, _y: i32) {
        self.show_panel();
    }

    async fn scroll(&self, _delta: i32, _orientation: &str) {}

    #[zbus(property)]
    fn category(&self) -> &str {
        "ApplicationStatus"
    }

    #[zbus(property)]
    fn id(&self) -> &str {
        "taskbar-todolist-desktop"
    }

    #[zbus(property)]
    fn title(&self) -> &str {
        "Taskbar Todolist"
    }

    #[zbus(property)]
    fn status(&self) -> &str {
        "Active"
    }

    #[zbus(property)]
    fn window_id(&self) -> u32 {
        0
    }

    #[zbus(property)]
    fn icon_name(&self) -> &str {
        "view-task"
    }
}

#[derive(Debug)]
pub enum TrayError {
    Bus(zbus::Error),
    MissingUniqueBusName,
    Runtime(std::io::Error),
    WatcherUnavailable(zbus::Error),
}

impl fmt::Display for TrayError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bus(error) => write!(formatter, "StatusNotifierItem D-Bus error: {error}"),
            Self::MissingUniqueBusName => {
                write!(
                    formatter,
                    "StatusNotifierItem connection has no unique bus name"
                )
            }
            Self::Runtime(error) => write!(formatter, "tray async runtime error: {error}"),
            Self::WatcherUnavailable(error) => {
                write!(formatter, "StatusNotifierWatcher is unavailable: {error}")
            }
        }
    }
}

impl Error for TrayError {}

impl From<zbus::Error> for TrayError {
    fn from(error: zbus::Error) -> Self {
        if is_watcher_unavailable(&error) {
            Self::WatcherUnavailable(error)
        } else {
            Self::Bus(error)
        }
    }
}

fn is_watcher_unavailable(error: &zbus::Error) -> bool {
    let message = error.to_string();
    message.contains(WATCHER_BUS_NAME) && message.contains("ServiceUnknown")
}
