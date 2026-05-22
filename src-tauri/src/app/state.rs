use std::fs;

use tauri::{App, Manager};

use crate::app::errors::StartupError;
use crate::tasks::repository::TaskRepository;

pub struct AppState {
    tasks: TaskRepository,
}

impl AppState {
    pub fn tasks(&self) -> &TaskRepository {
        &self.tasks
    }
}

pub fn initialize(app: &mut App) -> Result<AppState, StartupError> {
    let app_data_dir = app.path().app_data_dir()?;
    fs::create_dir_all(&app_data_dir)?;
    let database_path = app_data_dir.join("tasks.sqlite3");

    let tasks = tauri::async_runtime::block_on(TaskRepository::connect(database_path))?;

    Ok(AppState { tasks })
}
