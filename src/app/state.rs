use crate::app::errors::AppError;
use crate::tasks::TaskService;

#[derive(Clone)]
pub struct AppState {
    pub tasks: TaskService,
}

impl AppState {
    pub fn new() -> Result<Self, AppError> {
        Ok(Self {
            tasks: TaskService::new()?,
        })
    }

    #[cfg(test)]
    pub(crate) fn in_memory() -> Result<Self, AppError> {
        Ok(Self {
            tasks: TaskService::in_memory()?,
        })
    }
}
