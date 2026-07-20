use crate::app::errors::AppError;
use crate::app::settings::AppSettings;
use crate::tasks::TaskService;

#[derive(Clone)]
pub struct AppState {
    pub settings: AppSettings,
    pub tasks: TaskService,
}

impl AppState {
    pub fn new() -> Result<Self, AppError> {
        Ok(Self {
            settings: AppSettings::load_or_normalize()?,
            tasks: TaskService::new()?,
        })
    }

    #[cfg(test)]
    pub(crate) fn in_memory() -> Result<Self, AppError> {
        Ok(Self {
            settings: AppSettings::default(),
            tasks: TaskService::in_memory()?,
        })
    }
}
