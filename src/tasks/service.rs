use crate::app::errors::AppError;
use crate::tasks::model::Task;
use crate::tasks::repository::TaskRepository;

#[derive(Clone)]
pub struct TaskService {
    repository: TaskRepository,
}

impl TaskService {
    pub fn new() -> Self {
        Self {
            repository: TaskRepository::new(),
        }
    }

    pub fn create_task(&self, text: String) -> Result<Task, AppError> {
        self.repository.create_placeholder(text)
    }
}
