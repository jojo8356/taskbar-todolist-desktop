use std::fs;

use tauri::{App, Manager};

use crate::tasks::repository::TaskRepository;

pub struct AppState {
    pub tasks: TaskRepository,
}

pub fn initialize(app: &mut App) -> tauri::Result<AppState> {
    let app_data_dir = app.path().app_data_dir()?;
    fs::create_dir_all(&app_data_dir)?;
    let database_path = app_data_dir.join("tasks.sqlite3");

    let tasks = tauri::async_runtime::block_on(TaskRepository::connect(database_path))?;

    Ok(AppState { tasks })
}
