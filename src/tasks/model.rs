//! Core task data types shared by the repository, service, and UI model.

/// Stored task status.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskStatus {
    Todo,
    Done,
}

impl TaskStatus {
    /// Stable string representation stored in SQLite and passed to Slint.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Todo => "todo",
            Self::Done => "done",
        }
    }

    /// Parses a stored or UI-provided status value.
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "todo" => Some(Self::Todo),
            "done" => Some(Self::Done),
            _ => None,
        }
    }
}

/// Local todo item persisted in SQLite.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub id: String,
    pub text: String,
    pub status: TaskStatus,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}
