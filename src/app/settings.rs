//! YAML-backed user preferences and popup sizing rules.
//!
//! The app keeps settings local and self-healing: missing, malformed, or
//! out-of-range files are rewritten with normalized values. The popup height is
//! derived from `visible_tasks`, whose runtime maximum is computed from the
//! current screen height.

use crate::app::errors::{AppError, AppErrorCode};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

const SETTINGS_FILE_NAME: &str = "taskbar-todolist.settings.yaml";
pub const DEFAULT_VISIBLE_TASKS: i32 = 3;
pub const MIN_VISIBLE_TASKS: i32 = 1;
/// Conservative cap used before GTK can report the actual screen height.
pub const FALLBACK_MAX_VISIBLE_TASKS: i32 = 20;
/// Effective row pitch in the Slint list: 34px row height plus 6px spacing.
pub const TASK_ROW_HEIGHT_PX: i32 = 42;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppSettings {
    pub language: AppLanguage,
    /// User-selected number of task rows shown before the list scrolls.
    pub visible_tasks: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AppLanguage {
    Fr,
    En,
}

impl AppSettings {
    /// Loads settings from the current working directory and rewrites them when
    /// they are missing or invalid.
    pub fn load_or_normalize() -> Result<Self, AppError> {
        Self::load_or_normalize_from_path(default_settings_path())
    }

    pub fn load_or_normalize_from_path(path: impl AsRef<Path>) -> Result<Self, AppError> {
        let path = path.as_ref();
        let settings = read_settings(path)
            .map(|settings| normalize_settings(settings, FALLBACK_MAX_VISIBLE_TASKS))
            .unwrap_or_else(|| Self::default());
        write_settings(path, &settings)?;
        Ok(settings)
    }

    /// Persists settings using the runtime task limit detected for this screen.
    pub fn save_with_limit(&self, visible_task_limit: i32) -> Result<(), AppError> {
        write_settings(
            default_settings_path(),
            &normalize_settings(self.clone(), visible_task_limit),
        )
    }
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            language: AppLanguage::Fr,
            visible_tasks: DEFAULT_VISIBLE_TASKS,
        }
    }
}

impl AppLanguage {
    pub fn from_ui_code(value: &str) -> Option<Self> {
        match value.to_lowercase().as_str() {
            "fr" => Some(Self::Fr),
            "en" => Some(Self::En),
            _ => None,
        }
    }

    pub fn ui_code(self) -> &'static str {
        match self {
            Self::Fr => "fr",
            Self::En => "en",
        }
    }
}

fn default_settings_path() -> PathBuf {
    PathBuf::from(SETTINGS_FILE_NAME)
}

fn read_settings(path: &Path) -> Option<AppSettings> {
    let content = fs::read_to_string(path).ok()?;
    serde_yaml::from_str::<AppSettings>(&content).ok()
}

/// Returns the maximum visible row count for the current display height.
pub fn intelligent_visible_task_limit(screen_height: i32) -> i32 {
    (screen_height / TASK_ROW_HEIGHT_PX).max(MIN_VISIBLE_TASKS)
}

/// Clamps a user-entered row count to the active runtime limit.
pub fn normalize_visible_tasks(visible_tasks: i32, visible_task_limit: i32) -> i32 {
    visible_tasks.clamp(MIN_VISIBLE_TASKS, visible_task_limit.max(MIN_VISIBLE_TASKS))
}

fn normalize_settings(mut settings: AppSettings, visible_task_limit: i32) -> AppSettings {
    settings.visible_tasks = normalize_visible_tasks(settings.visible_tasks, visible_task_limit);
    settings
}

fn write_settings(path: impl AsRef<Path>, settings: &AppSettings) -> Result<(), AppError> {
    let content = serde_yaml::to_string(settings).map_err(settings_error)?;
    fs::write(path, content).map_err(settings_error)
}

fn settings_error(error: impl std::fmt::Display) -> AppError {
    AppError::new(AppErrorCode::TaskStorageError, error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn missing_settings_file_is_created_with_defaults() {
        let path = temp_settings_path();

        let settings = AppSettings::load_or_normalize_from_path(&path).unwrap();
        let content = fs::read_to_string(&path).unwrap();

        assert_eq!(settings, AppSettings::default());
        assert!(content.contains("language: fr"));
        assert!(content.contains("visible_tasks: 3"));
        let _ = fs::remove_file(path);
    }

    #[test]
    fn invalid_settings_file_is_replaced_with_defaults() {
        let path = temp_settings_path();
        fs::write(&path, "language: de\nvisible_tasks: beaucoup\n").unwrap();

        let settings = AppSettings::load_or_normalize_from_path(&path).unwrap();
        let content = fs::read_to_string(&path).unwrap();

        assert_eq!(settings, AppSettings::default());
        assert!(content.contains("language: fr"));
        assert!(content.contains("visible_tasks: 3"));
        let _ = fs::remove_file(path);
    }

    #[test]
    fn out_of_range_visible_tasks_is_clamped_and_rewritten() {
        let path = temp_settings_path();
        fs::write(&path, "language: en\nvisible_tasks: 999\n").unwrap();

        let settings = AppSettings::load_or_normalize_from_path(&path).unwrap();
        let content = fs::read_to_string(&path).unwrap();

        assert_eq!(settings.language, AppLanguage::En);
        assert_eq!(settings.visible_tasks, FALLBACK_MAX_VISIBLE_TASKS);
        assert!(content.contains("language: en"));
        assert!(content.contains("visible_tasks: 20"));
        let _ = fs::remove_file(path);
    }

    #[test]
    fn intelligent_visible_task_limit_uses_screen_height_over_row_height() {
        assert_eq!(intelligent_visible_task_limit(1080), 25);
        assert_eq!(intelligent_visible_task_limit(800), 19);
        assert_eq!(intelligent_visible_task_limit(12), MIN_VISIBLE_TASKS);
    }

    #[test]
    fn visible_tasks_are_clamped_to_runtime_limit() {
        assert_eq!(normalize_visible_tasks(999, 25), 25);
        assert_eq!(normalize_visible_tasks(0, 25), MIN_VISIBLE_TASKS);
        assert_eq!(normalize_visible_tasks(5, 0), MIN_VISIBLE_TASKS);
    }

    fn temp_settings_path() -> PathBuf {
        std::env::temp_dir().join(format!("taskbar-todolist-settings-{}.yaml", Uuid::new_v4()))
    }
}
