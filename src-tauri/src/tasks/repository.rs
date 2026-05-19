use std::{path::Path, str::FromStr};

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

    pub async fn create_for_smoke_test(&self, text: &str) -> Result<Task, sqlx::Error> {
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
        .bind(text)
        .bind(status)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await?;

        self.get_by_id(&id).await
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Task, sqlx::Error> {
        sqlx::query_as::<_, Task>(
            r#"
            SELECT id, text, status, created_at, updated_at, deleted_at
            FROM tasks
            WHERE id = ?1
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn creates_and_reads_task_with_stable_fields() {
        let repository = TaskRepository::connect_in_memory()
            .await
            .expect("in-memory repository should initialize");

        let task = repository
            .create_for_smoke_test("Capture a local task")
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
    }
}
