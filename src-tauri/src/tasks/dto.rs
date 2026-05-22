use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::tasks::model::{Task, TaskStatus};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskDto {
    pub id: String,
    pub text: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

impl From<Task> for TaskDto {
    fn from(task: Task) -> Self {
        Self {
            id: task.id,
            text: task.text,
            status: task.status,
            created_at: iso_utc(task.created_at),
            updated_at: iso_utc(task.updated_at),
            deleted_at: task.deleted_at.map(iso_utc),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatusDto {
    Todo,
    Done,
}

impl From<TaskStatusDto> for TaskStatus {
    fn from(status: TaskStatusDto) -> Self {
        match status {
            TaskStatusDto::Todo => Self::Todo,
            TaskStatusDto::Done => Self::Done,
        }
    }
}

fn iso_utc(value: DateTime<Utc>) -> String {
    value.to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
}
