use serde::Serialize;
use std::{error::Error, fmt};

use crate::tasks::repository::TaskRepositoryError;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppError {
    pub code: &'static str,
    pub message: String,
}

impl AppError {
    pub fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    pub fn task_text_empty() -> Self {
        Self::new("TASK_TEXT_EMPTY", "Task text cannot be empty.")
    }

    pub fn task_not_found() -> Self {
        Self::new("TASK_NOT_FOUND", "Task not found.")
    }

    pub fn task_validation_error(message: impl Into<String>) -> Self {
        Self::new("TASK_VALIDATION_ERROR", message)
    }

    pub fn task_storage_error() -> Self {
        Self::new("TASK_STORAGE_ERROR", "Task storage operation failed.")
    }

    pub fn app_internal_error() -> Self {
        Self::new("APP_INTERNAL_ERROR", "Internal application error.")
    }
}

impl From<TaskRepositoryError> for AppError {
    fn from(error: TaskRepositoryError) -> Self {
        match error {
            TaskRepositoryError::EmptyText => Self::task_text_empty(),
            TaskRepositoryError::InvalidStatus => {
                Self::task_validation_error("Task status is invalid.")
            }
            TaskRepositoryError::NotFound => Self::task_not_found(),
            TaskRepositoryError::Storage(_) => Self::task_storage_error(),
        }
    }
}

pub enum StartupError {
    Io(std::io::Error),
    Path(tauri::Error),
    Storage(sqlx::Error),
}

impl fmt::Debug for StartupError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, formatter)
    }
}

impl fmt::Display for StartupError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "I/O startup error: {error}"),
            Self::Path(error) => write!(formatter, "Application path startup error: {error}"),
            Self::Storage(error) => write!(formatter, "Task storage startup error: {error}"),
        }
    }
}

impl Error for StartupError {}

impl From<std::io::Error> for StartupError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<tauri::Error> for StartupError {
    fn from(error: tauri::Error) -> Self {
        Self::Path(error)
    }
}

impl From<sqlx::Error> for StartupError {
    fn from(error: sqlx::Error) -> Self {
        Self::Storage(error)
    }
}
