use std::{error::Error, fmt};

use tauri::{App, AppHandle};
use zbus::{interface, Connection};

use crate::app::windows;

const ITEM_PATH: &str = "/StatusNotifierItem";
const WATCHER_BUS_NAME: &str = "org.kde.StatusNotifierWatcher";
const WATCHER_PATH: &str = "/StatusNotifierWatcher";
const WATCHER_INTERFACE: &str = "org.kde.StatusNotifierWatcher";

pub struct StatusNotifierTray {
    _connection: Connection,
}

pub fn create_tray(app: &mut App) -> Result<Option<StatusNotifierTray>, TrayError> {
    let app_handle = app.handle().clone();

    let tray = tauri::async_runtime::block_on(async move {
        let connection = Connection::session().await?;
        connection
            .object_server()
            .at(ITEM_PATH, StatusNotifierItem { app: app_handle })
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

        Ok::<StatusNotifierTray, TrayError>(StatusNotifierTray {
            _connection: connection,
        })
    });

    match tray {
        Ok(tray) => Ok(Some(tray)),
        Err(TrayError::WatcherUnavailable(_)) => Ok(None),
        Err(error) => Err(error),
    }
}

struct StatusNotifierItem {
    app: AppHandle,
}

#[interface(name = "org.kde.StatusNotifierItem")]
impl StatusNotifierItem {
    async fn activate(&self, _x: i32, _y: i32) {
        let _ = windows::show_tray_panel(&self.app);
    }

    async fn context_menu(&self, _x: i32, _y: i32) {
        let _ = windows::show_tray_panel(&self.app);
    }

    async fn secondary_activate(&self, _x: i32, _y: i32) {
        let _ = windows::show_tray_panel(&self.app);
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
