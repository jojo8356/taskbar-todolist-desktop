use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum AppErrorCode {
    TaskTextEmpty,
    TaskNotFound,
    TaskStorageError,
    TaskValidationError,
    AppInternalError,
}

#[derive(Debug, Clone)]
pub struct AppError {
    pub code: AppErrorCode,
    pub message: String,
}

impl AppError {
    pub fn new(code: AppErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.code, self.message)
    }
}

impl std::error::Error for AppError {}
