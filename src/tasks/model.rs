#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum TaskStatus {
    Todo,
    Done,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub id: String,
    pub text: String,
    pub status: TaskStatus,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}
