use crate::tasks::TaskService;

#[derive(Clone)]
pub struct AppState {
    pub tasks: TaskService,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            tasks: TaskService::new(),
        }
    }
}
