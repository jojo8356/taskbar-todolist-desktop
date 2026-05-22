use std::{error::Error, fmt, path::Path, str::FromStr};

use chrono::Utc;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    SqlitePool,
};
use uuid::Uuid;

use crate::tasks::{
    migrations,
    model::{Task, TaskStatus},
};

#[derive(Debug)]
pub enum TaskRepositoryError {
    EmptyText,
    InvalidStatus,
    NotFound,
    Storage(sqlx::Error),
}

impl fmt::Display for TaskRepositoryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyText => write!(formatter, "task text cannot be empty"),
            Self::InvalidStatus => write!(formatter, "task status is invalid"),
            Self::NotFound => write!(formatter, "task not found"),
            Self::Storage(error) => write!(formatter, "task storage error: {error}"),
        }
    }
}

impl Error for TaskRepositoryError {}

impl From<sqlx::Error> for TaskRepositoryError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => Self::NotFound,
            error => Self::Storage(error),
        }
    }
}

#[derive(Clone)]
pub struct TaskRepository {
    pool: SqlitePool,
}

impl TaskRepository {
    pub async fn connect(database_path: impl AsRef<Path>) -> Result<Self, sqlx::Error> {
        let database_url = format!("sqlite://{}", database_path.as_ref().display());
        let options = SqliteConnectOptions::from_str(&database_url)?.create_if_missing(true);
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await?;

        migrations::run(&pool).await?;

        Ok(Self { pool })
    }

    #[cfg(test)]
    pub async fn connect_in_memory() -> Result<Self, sqlx::Error> {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await?;

        migrations::run(&pool).await?;

        Ok(Self { pool })
    }

    pub async fn create(&self, text: &str) -> Result<Task, TaskRepositoryError> {
        let text = validate_text(text)?;
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let status = TaskStatus::Todo.as_str();

        sqlx::query(
            r#"
            INSERT INTO tasks (id, text, status, created_at, updated_at, deleted_at)
            VALUES (?1, ?2, ?3, ?4, ?5, NULL)
            "#,
        )
        .bind(&id)
        .bind(&text)
        .bind(status)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await?;

        self.get_by_id(&id).await
    }

    pub async fn list_active(&self) -> Result<Vec<Task>, TaskRepositoryError> {
        let tasks = sqlx::query_as::<_, Task>(
            r#"
            SELECT id, text, status, created_at, updated_at, deleted_at
            FROM tasks
            WHERE deleted_at IS NULL
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tasks)
    }

    pub async fn update(
        &self,
        id: &str,
        text: Option<&str>,
        status: Option<TaskStatus>,
    ) -> Result<Task, TaskRepositoryError> {
        let existing = self.get_active_by_id(id).await?;
        let next_text = match text {
            Some(text) => validate_text(text)?,
            None => existing.text,
        };
        let next_status = status
            .map(TaskStatus::as_str)
            .unwrap_or(existing.status.as_str());

        if TaskStatus::from_str(next_status).is_err() {
            return Err(TaskRepositoryError::InvalidStatus);
        }

        let now = Utc::now();

        sqlx::query(
            r#"
            UPDATE tasks
            SET text = ?2, status = ?3, updated_at = ?4
            WHERE id = ?1 AND deleted_at IS NULL
            "#,
        )
        .bind(id)
        .bind(next_text)
        .bind(next_status)
        .bind(now)
        .execute(&self.pool)
        .await?;

        self.get_by_id(id).await
    }

    pub async fn soft_delete(&self, id: &str) -> Result<(), TaskRepositoryError> {
        let now = Utc::now();
        let result = sqlx::query(
            r#"
            UPDATE tasks
            SET deleted_at = ?2, updated_at = ?2
            WHERE id = ?1 AND deleted_at IS NULL
            "#,
        )
        .bind(id)
        .bind(now)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(TaskRepositoryError::NotFound);
        }

        Ok(())
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Task, TaskRepositoryError> {
        let task = sqlx::query_as::<_, Task>(
            r#"
            SELECT id, text, status, created_at, updated_at, deleted_at
            FROM tasks
            WHERE id = ?1
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(task)
    }

    async fn get_active_by_id(&self, id: &str) -> Result<Task, TaskRepositoryError> {
        let task = sqlx::query_as::<_, Task>(
            r#"
            SELECT id, text, status, created_at, updated_at, deleted_at
            FROM tasks
            WHERE id = ?1 AND deleted_at IS NULL
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(task)
    }
}

fn validate_text(text: &str) -> Result<String, TaskRepositoryError> {
    let text = text.trim();

    if text.is_empty() {
        return Err(TaskRepositoryError::EmptyText);
    }

    Ok(text.to_string())
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;

    #[test]
    fn creates_and_reads_task_with_stable_fields() {
        tauri::async_runtime::block_on(async {
            let repository = TaskRepository::connect_in_memory()
                .await
                .expect("in-memory repository should initialize");

            let task = repository
                .create("Capture a local task")
                .await
                .expect("task should be created");

            let saved = repository
                .get_by_id(&task.id)
                .await
                .expect("task should be readable");

            assert_eq!(saved.id, task.id);
            assert_eq!(saved.text, "Capture a local task");
            assert_eq!(saved.status, TaskStatus::Todo.as_str());
            assert_eq!(saved.created_at, task.created_at);
            assert_eq!(saved.updated_at, task.updated_at);
            assert!(saved.deleted_at.is_none());
        });
    }

    #[test]
    fn trims_and_rejects_empty_task_text() {
        tauri::async_runtime::block_on(async {
            let repository = repository().await;

            let task = repository
                .create("  Trimmed task  ")
                .await
                .expect("task should be created");
            assert_eq!(task.text, "Trimmed task");

            let error = repository
                .create("  ")
                .await
                .expect_err("empty task text should be rejected");
            assert!(matches!(error, TaskRepositoryError::EmptyText));
        });
    }

    #[test]
    fn lists_only_active_tasks_newest_first() {
        tauri::async_runtime::block_on(async {
            let repository = repository().await;
            let first = repository.create("First").await.expect("first task");
            let second = repository.create("Second").await.expect("second task");
            repository
                .soft_delete(&first.id)
                .await
                .expect("first task should be soft-deleted");

            let tasks = repository
                .list_active()
                .await
                .expect("active tasks should list");

            assert_eq!(tasks.len(), 1);
            assert_eq!(tasks[0].id, second.id);
        });
    }

    #[test]
    fn updates_task_text_and_status() {
        tauri::async_runtime::block_on(async {
            let repository = repository().await;
            let task = repository.create("Draft").await.expect("task");

            let updated = repository
                .update(&task.id, Some("Done task"), Some(TaskStatus::Done))
                .await
                .expect("task should update");

            assert_eq!(updated.id, task.id);
            assert_eq!(updated.text, "Done task");
            assert_eq!(updated.status, TaskStatus::Done.as_str());
            assert!(updated.updated_at >= task.updated_at);
        });
    }

    #[test]
    fn returns_not_found_for_missing_update_and_delete() {
        tauri::async_runtime::block_on(async {
            let repository = repository().await;

            let update_error = repository
                .update("missing", Some("Nope"), None)
                .await
                .expect_err("missing task update should fail");
            assert!(matches!(update_error, TaskRepositoryError::NotFound));

            let delete_error = repository
                .soft_delete("missing")
                .await
                .expect_err("missing task delete should fail");
            assert!(matches!(delete_error, TaskRepositoryError::NotFound));
        });
    }

    #[test]
    fn failed_update_and_delete_do_not_mutate_existing_task() {
        tauri::async_runtime::block_on(async {
            let repository = repository().await;
            let task = repository.create("Stable").await.expect("task");

            let empty_text_error = repository
                .update(&task.id, Some(" "), None)
                .await
                .expect_err("empty update should fail");
            assert!(matches!(empty_text_error, TaskRepositoryError::EmptyText));

            let delete_error = repository
                .soft_delete("missing")
                .await
                .expect_err("missing task delete should fail");
            assert!(matches!(delete_error, TaskRepositoryError::NotFound));

            let saved = repository
                .get_by_id(&task.id)
                .await
                .expect("existing task should remain");
            assert_eq!(saved.text, "Stable");
            assert_eq!(saved.status, TaskStatus::Todo.as_str());
            assert!(saved.deleted_at.is_none());
        });
    }

    #[test]
    fn failed_create_does_not_insert_task() {
        tauri::async_runtime::block_on(async {
            let repository = repository().await;

            let error = repository
                .create(" ")
                .await
                .expect_err("empty create should fail");
            assert!(matches!(error, TaskRepositoryError::EmptyText));
            assert!(repository
                .list_active()
                .await
                .expect("active list")
                .is_empty());
        });
    }

    #[test]
    fn soft_delete_keeps_tombstone() {
        tauri::async_runtime::block_on(async {
            let repository = repository().await;
            let task = repository.create("Remove me").await.expect("task");

            repository
                .soft_delete(&task.id)
                .await
                .expect("task should be soft-deleted");

            let saved = repository
                .get_by_id(&task.id)
                .await
                .expect("deleted task should remain in storage");
            assert!(saved.deleted_at.is_some());
            assert!(repository
                .list_active()
                .await
                .expect("active list")
                .is_empty());
        });
    }

    #[test]
    fn lists_500_active_tasks_under_500_ms() {
        tauri::async_runtime::block_on(async {
            let repository = repository().await;

            for index in 0..500 {
                repository
                    .create(&format!("Task {index}"))
                    .await
                    .expect("task should be created");
            }

            let start = Instant::now();
            let tasks = repository
                .list_active()
                .await
                .expect("active tasks should list");
            let elapsed = start.elapsed();

            assert_eq!(tasks.len(), 500);
            assert!(
                elapsed.as_millis() < 500,
                "active task list loaded in {elapsed:?}"
            );
        });
    }

    async fn repository() -> TaskRepository {
        TaskRepository::connect_in_memory()
            .await
            .expect("in-memory repository should initialize")
    }
}
