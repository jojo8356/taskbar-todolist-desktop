//! Task application service.
//!
//! This layer keeps UI callbacks small and centralizes logging around repository
//! operations, while the repository remains responsible for storage details.

use crate::app::errors::AppError;
use crate::tasks::model::{Task, TaskStatus};
use crate::tasks::repository::TaskRepository;

#[derive(Clone)]
pub struct TaskService {
    repository: TaskRepository,
}

impl TaskService {
    /// Opens the default SQLite-backed task repository.
    pub fn new() -> Result<Self, AppError> {
        Ok(Self {
            repository: TaskRepository::new()?,
        })
    }

    #[cfg(test)]
    pub(crate) fn in_memory() -> Result<Self, AppError> {
        Ok(Self {
            repository: TaskRepository::in_memory()?,
        })
    }

    /// Creates a todo task after repository-level validation.
    pub fn create_task(&self, text: String) -> Result<Task, AppError> {
        tracing::trace!(text = %text, "TaskService::create_task");
        let result = self.repository.create(text);
        if let Ok(task) = &result {
            tracing::debug!(task_id = %task.id, text = %task.text, status = %task.status.as_str(), "task created");
        }
        result
    }

    /// Lists active non-deleted tasks, with completed tasks ordered last.
    pub fn list_active_tasks(&self) -> Result<Vec<Task>, AppError> {
        tracing::trace!("TaskService::list_active_tasks");
        let result = self.repository.list_active();
        match &result {
            Ok(tasks) => tracing::debug!(count = tasks.len(), "active tasks listed"),
            Err(error) => tracing::error!(error = %error, "failed to list active tasks"),
        }
        result
    }

    /// Updates task text and/or status.
    pub fn update_task(
        &self,
        id: String,
        text: Option<String>,
        status: Option<TaskStatus>,
    ) -> Result<Task, AppError> {
        tracing::trace!(
            task_id = %id,
            text = ?text,
            status = ?status.as_ref().map(TaskStatus::as_str),
            "TaskService::update_task"
        );
        let result = self.repository.update(id, text, status);
        match &result {
            Ok(task) => {
                tracing::debug!(task_id = %task.id, text = %task.text, status = %task.status.as_str(), "task updated")
            }
            Err(error) => tracing::error!(error = %error, "failed to update task"),
        }
        result
    }

    /// Soft-deletes a task so it disappears from the active list.
    pub fn delete_task(&self, id: String) -> Result<(), AppError> {
        tracing::trace!(task_id = %id, "TaskService::delete_task");
        let result = self.repository.soft_delete(id);
        match &result {
            Ok(()) => tracing::debug!("task deleted"),
            Err(error) => tracing::error!(error = %error, "failed to delete task"),
        }
        result
    }
}
