use crate::app::errors::{AppError, AppErrorCode};
use crate::tasks::model::{Task, TaskStatus};

#[derive(Clone, Default)]
pub struct TaskRepository;

impl TaskRepository {
    pub fn new() -> Self {
        Self
    }

    pub fn create_placeholder(&self, text: String) -> Result<Task, AppError> {
        let text = text.trim().to_string();
        if text.is_empty() {
            return Err(AppError::new(
                AppErrorCode::TaskTextEmpty,
                "Task text cannot be empty",
            ));
        }

        Ok(Task {
            id: "scaffold-task".to_string(),
            text,
            status: TaskStatus::Todo,
            created_at: "1970-01-01T00:00:00Z".to_string(),
            updated_at: "1970-01-01T00:00:00Z".to_string(),
            deleted_at: None,
        })
    }
}
