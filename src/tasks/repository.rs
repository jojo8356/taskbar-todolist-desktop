use crate::app::errors::{AppError, AppErrorCode};
use crate::tasks::model::{Task, TaskStatus};
use chrono::Utc;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{Row, SqlitePool};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use tokio::runtime::Runtime;
use uuid::Uuid;

#[derive(Clone)]
pub struct TaskRepository {
    runtime: Arc<Runtime>,
    pool: SqlitePool,
}

impl TaskRepository {
    pub fn new() -> Result<Self, AppError> {
        Self::with_database_path(default_database_path())
    }

    #[cfg(test)]
    pub(crate) fn in_memory() -> Result<Self, AppError> {
        Self::with_database_path(":memory:")
    }

    fn with_database_path(path: impl Into<PathBuf>) -> Result<Self, AppError> {
        let runtime = Arc::new(Runtime::new().map_err(storage_error)?);
        let path = path.into();
        tracing::debug!(path = %path.display(), "opening task repository");
        let options = if path == PathBuf::from(":memory:") {
            SqliteConnectOptions::from_str("sqlite::memory:").map_err(storage_error)?
        } else {
            SqliteConnectOptions::new()
                .filename(path)
                .create_if_missing(true)
        };

        let pool = runtime
            .block_on(async {
                let pool = SqlitePoolOptions::new()
                    .max_connections(1)
                    .connect_with(options)
                    .await?;
                crate::tasks::migrations::MIGRATOR.run(&pool).await?;
                Ok::<_, sqlx::Error>(pool)
            })
            .map_err(storage_error)?;

        Ok(Self { runtime, pool })
    }

    pub fn create(&self, text: String) -> Result<Task, AppError> {
        tracing::trace!(text = %text, "TaskRepository::create");
        let text = validate_text(text)?;
        let now = current_timestamp();
        let task = Task {
            id: Uuid::new_v4().to_string(),
            text,
            status: TaskStatus::Todo,
            created_at: now.clone(),
            updated_at: now,
            deleted_at: None,
        };

        self.runtime
            .block_on(async {
                sqlx::query(
                    "INSERT INTO tasks (id, text, status, created_at, updated_at, deleted_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, NULL)",
                )
                .bind(&task.id)
                .bind(&task.text)
                .bind(task.status.as_str())
                .bind(&task.created_at)
                .bind(&task.updated_at)
                .execute(&self.pool)
                .await
            })
            .map_err(storage_error)?;

        Ok(task)
    }

    pub fn list_active(&self) -> Result<Vec<Task>, AppError> {
        tracing::trace!("TaskRepository::list_active");
        self.runtime
            .block_on(async {
                sqlx::query(
                    "SELECT id, text, status, created_at, updated_at, deleted_at
                 FROM tasks
                 WHERE deleted_at IS NULL
                 ORDER BY CASE status WHEN 'todo' THEN 0 ELSE 1 END, created_at ASC",
                )
                .fetch_all(&self.pool)
                .await
            })
            .map_err(storage_error)?
            .into_iter()
            .map(task_from_row)
            .collect()
    }

    pub fn update(
        &self,
        id: String,
        text: Option<String>,
        status: Option<TaskStatus>,
    ) -> Result<Task, AppError> {
        tracing::trace!(
            task_id = %id,
            text = ?text,
            status = ?status.as_ref().map(TaskStatus::as_str),
            "TaskRepository::update"
        );
        let current = self.get_active(&id)?;
        let text = match text {
            Some(text) => validate_text(text)?,
            None => current.text,
        };
        let status = status.unwrap_or(current.status);
        let updated_at = current_timestamp();

        let affected = self
            .runtime
            .block_on(async {
                sqlx::query(
                    "UPDATE tasks
                 SET text = ?1, status = ?2, updated_at = ?3
                 WHERE id = ?4 AND deleted_at IS NULL",
                )
                .bind(&text)
                .bind(status.as_str())
                .bind(&updated_at)
                .bind(&id)
                .execute(&self.pool)
                .await
            })
            .map_err(storage_error)?
            .rows_affected();

        if affected == 0 {
            return Err(not_found());
        }

        self.get_active(&id)
    }

    pub fn soft_delete(&self, id: String) -> Result<(), AppError> {
        tracing::trace!(task_id = %id, "TaskRepository::soft_delete");
        let deleted_at = current_timestamp();
        let affected = self
            .runtime
            .block_on(async {
                sqlx::query(
                    "UPDATE tasks
                 SET deleted_at = ?1, updated_at = ?1
                 WHERE id = ?2 AND deleted_at IS NULL",
                )
                .bind(&deleted_at)
                .bind(&id)
                .execute(&self.pool)
                .await
            })
            .map_err(storage_error)?
            .rows_affected();

        if affected == 0 {
            Err(not_found())
        } else {
            Ok(())
        }
    }

    fn get_active(&self, id: &str) -> Result<Task, AppError> {
        self.runtime
            .block_on(async {
                sqlx::query(
                    "SELECT id, text, status, created_at, updated_at, deleted_at
                 FROM tasks
                 WHERE id = ?1 AND deleted_at IS NULL",
                )
                .bind(id)
                .fetch_optional(&self.pool)
                .await
            })
            .map_err(storage_error)?
            .map(task_from_row)
            .transpose()?
            .ok_or_else(not_found)
    }
}

fn default_database_path() -> PathBuf {
    PathBuf::from("taskbar-todolist.sqlite")
}

fn validate_text(text: String) -> Result<String, AppError> {
    let text = text.trim().to_string();
    if text.is_empty() {
        Err(AppError::new(
            AppErrorCode::TaskTextEmpty,
            "Task text cannot be empty",
        ))
    } else {
        Ok(text)
    }
}

fn task_from_row(row: sqlx::sqlite::SqliteRow) -> Result<Task, AppError> {
    let status: String = row.try_get("status").map_err(storage_error)?;
    let status = TaskStatus::parse(&status).ok_or_else(|| {
        AppError::new(
            AppErrorCode::TaskStorageError,
            "Stored task status is invalid",
        )
    })?;

    Ok(Task {
        id: row.try_get("id").map_err(storage_error)?,
        text: row.try_get("text").map_err(storage_error)?,
        status,
        created_at: row.try_get("created_at").map_err(storage_error)?,
        updated_at: row.try_get("updated_at").map_err(storage_error)?,
        deleted_at: row.try_get("deleted_at").map_err(storage_error)?,
    })
}

fn current_timestamp() -> String {
    Utc::now().to_rfc3339()
}

fn not_found() -> AppError {
    AppError::new(AppErrorCode::TaskNotFound, "Task not found")
}

fn storage_error(error: impl std::fmt::Display) -> AppError {
    AppError::new(AppErrorCode::TaskStorageError, error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    #[test]
    fn create_and_list_active_tasks() {
        let repository = TaskRepository::in_memory().unwrap();

        let task = repository.create("Buy milk".to_string()).unwrap();
        let tasks = repository.list_active().unwrap();

        assert_eq!(tasks, vec![task]);
    }

    #[test]
    fn empty_text_is_rejected() {
        let repository = TaskRepository::in_memory().unwrap();

        let error = repository.create("   ".to_string()).unwrap_err();

        assert_eq!(error.code, AppErrorCode::TaskTextEmpty);
    }

    #[test]
    fn update_changes_text_and_status() {
        let repository = TaskRepository::in_memory().unwrap();
        let task = repository.create("Draft".to_string()).unwrap();

        let updated = repository
            .update(
                task.id.clone(),
                Some("Done task".to_string()),
                Some(TaskStatus::Done),
            )
            .unwrap();

        assert_eq!(updated.text, "Done task");
        assert_eq!(updated.status, TaskStatus::Done);
    }

    #[test]
    fn list_active_sorts_done_tasks_last() {
        let repository = TaskRepository::in_memory().unwrap();
        let first = repository.create("first".to_string()).unwrap();
        let second = repository.create("second".to_string()).unwrap();
        let third = repository.create("third".to_string()).unwrap();

        repository
            .update(first.id.clone(), None, Some(TaskStatus::Done))
            .unwrap();

        let tasks = repository.list_active().unwrap();

        assert_eq!(tasks[0].id, second.id);
        assert_eq!(tasks[1].id, third.id);
        assert_eq!(tasks[2].id, first.id);
    }

    #[test]
    fn soft_delete_filters_active_list() {
        let repository = TaskRepository::in_memory().unwrap();
        let task = repository.create("Remove me".to_string()).unwrap();

        repository.soft_delete(task.id).unwrap();

        assert!(repository.list_active().unwrap().is_empty());
    }

    #[test]
    fn list_active_500_tasks_stays_fast() {
        let repository = TaskRepository::in_memory().unwrap();
        for index in 0..500 {
            repository.create(format!("Task {index}")).unwrap();
        }

        let started = Instant::now();
        let tasks = repository.list_active().unwrap();

        assert_eq!(tasks.len(), 500);
        assert!(started.elapsed() < Duration::from_millis(500));
    }
}
