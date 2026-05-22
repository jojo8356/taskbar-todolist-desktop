use tauri::State;

use crate::{
    app::{errors::AppError, state::AppState},
    tasks::{
        dto::{TaskDto, TaskStatusDto},
        model::TaskStatus,
    },
};

#[tauri::command]
pub async fn create_task(text: String, state: State<'_, AppState>) -> Result<TaskDto, AppError> {
    state
        .tasks()
        .create(&text)
        .await
        .map(TaskDto::from)
        .map_err(AppError::from)
}

#[tauri::command]
pub async fn list_tasks(state: State<'_, AppState>) -> Result<Vec<TaskDto>, AppError> {
    state
        .tasks()
        .list_active()
        .await
        .map(|tasks| tasks.into_iter().map(TaskDto::from).collect())
        .map_err(AppError::from)
}

#[tauri::command]
pub async fn update_task(
    id: String,
    text: Option<String>,
    status: Option<TaskStatusDto>,
    state: State<'_, AppState>,
) -> Result<TaskDto, AppError> {
    let status = status.map(TaskStatus::from);

    state
        .tasks()
        .update(&id, text.as_deref(), status)
        .await
        .map(TaskDto::from)
        .map_err(AppError::from)
}

#[tauri::command]
pub async fn delete_task(id: String, state: State<'_, AppState>) -> Result<(), AppError> {
    state.tasks().soft_delete(&id).await.map_err(AppError::from)
}
